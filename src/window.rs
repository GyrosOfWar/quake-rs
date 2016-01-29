// Export the right window implementation based on which OS we're running.
#[cfg(target_os="windows")] pub use self::win32::Win32Window as Window;
#[cfg(any(target_os="unix", target_os="linux"))] pub use self::nix::X11Window as Window;

use event::Event;

pub struct EventIter<'a> {
    window: &'a mut Window
}

impl<'a> Iterator for EventIter<'a> {
    type Item = Event;

    fn next(&mut self) -> Option<Event> {
        self.window.poll_event()
    }
}

pub trait WindowTrait {
    /// Creates the Window object and opens it with the specified size.
    fn open(x_size: i32, y_size: i32) -> Self;
    /// Clears the window, painting it black. (No colors anymore, I want them to turn black)
    fn clear(&mut self);
    /// Returns an event, if one is available.
    fn poll_event(&mut self) -> Option<Event>;
    // TODO it should be no problem to implement this in terms of
    // poll_event but I haven't figured out how to just yet
    /// Returns an iterator over the events for the window.
    fn events(&mut self) -> EventIter;
}

#[cfg(any(target_os="unix", target_os="linux"))]
mod nix {
    use event::Event;
    use super::WindowTrait;

    // TODO implement X11 window or something of the sort
    pub struct X11Window;

    impl WindowTrait for X11Window {
        fn open(x_size: i32, y_size: i32) -> X11Window {
            unimplemented!()
        }

        fn clear(&mut self) {
            unimplemented!()
        }

        fn poll_event(&mut self) -> Option<Event> {
            unimplemented!()
        }

        fn events(&mut self) -> EventIter { unimplemented!() }
    }
}

// Windows implementation of a window
#[cfg(target_os="windows")]
mod win32 {
    use event::*;
    use super::{WindowTrait, EventIter};

    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;
    use std::cell::RefCell;
    use std::sync::mpsc::{channel, Sender, Receiver};
    use std::{thread, mem};

    use std::ptr;
    use winapi::*;
    use kernel32::*;
    use user32::*;
    use gdi32::*;

    // Window title
    const WIN_CLASS_NAME: &'static str = "rsquake";

    thread_local! {
        pub static CONTEXT: RefCell<Option<WindowContext>> = RefCell::new(None)
    }

    /// Contains a handle to the window and a channel to send events to the window. 
    pub struct WindowContext {
        hwnd: HWND,
        send: Sender<Event>
    }

    /// This function converts Rust strings to UTF-16 strings understood by Windows.
    fn to_wchar(string: &str) -> Vec<u16> {
        OsStr::new(string).encode_wide().collect()
    }

    /// Since we're not using WinMain, this is the way to get the hInstance of the currently
    /// running process. 
    unsafe fn get_instance() -> HINSTANCE {
        let instance = GetModuleHandleW(ptr::null());
        if instance.is_null() {
            panic!("GetModuleHandleW error: {}", GetLastError());
        }

        instance
    }

    /// This function does the nasty Win32 API business to open a window.
    unsafe fn create_window(wnd_proc: WNDPROC, width: i32, height: i32) -> HWND {
        let instance = get_instance();
        let cursor = LoadCursorW(ptr::null_mut(), IDC_ARROW);
        let style = WS_OVERLAPPEDWINDOW | WS_VISIBLE;
        let class_name = to_wchar(WIN_CLASS_NAME).as_ptr();

        let wc = WNDCLASSW {
            style: 0,
            lpfnWndProc: wnd_proc,
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: instance,
            hIcon: ptr::null_mut(),
            hCursor: cursor,
            hbrBackground: ptr::null_mut(),
            lpszMenuName: ptr::null_mut(),
            lpszClassName: class_name
        };

        if RegisterClassW(&wc) == 0 {
            panic!("RegisterClassW error: {}", GetLastError());
        }

        CreateWindowExW(
            0,
            class_name,
            class_name,
            style,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            width,
            height,
            ptr::null_mut(),
            ptr::null_mut(),
            instance,
            ptr::null_mut()
        )
    }

    pub struct Win32Window {
        window: HWND,
        x_size: i32,
        y_size: i32,
        recv: Receiver<Event>
    }

    // This tells the Rust compiler that it's OK to send the Window 
    // to a different thread. Needs to be explicitly implemented, 
    // because raw pointers aren't Send per default (HWND is just a 
    // raw pointer)
    unsafe impl Send for Win32Window {}

    // The idea and implementation for passing events to the Window from the WNDPROC is
    // from the directx crate by Eljay, see here: https://github.com/Eljay/directx
    
    /// The WNDPROC callback. This is where programs will usually react to events by modifiying
    /// some global state. This implementation maps Windows messages to our own Event enum, which 
    /// then gets sent to the currently open Window. 
    unsafe extern "system" fn wnd_callback(hwnd: HWND, message: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        fn send_event(event: Event) -> LRESULT {
            CONTEXT.with(|cell| {
                if let Some(ref ctx) = *cell.borrow() {
                    ctx.send.send(event).ok();
                }
            });
            0
        }
        
        fn send_mouse_event(event: MouseEvent, lparam: LPARAM) -> LRESULT {
            send_event(Event::MouseInput(
                event,
                lparam as i16,
                (lparam >> 16) as i16
            ))
        }
        
        // Map windows virtual key to left/right versions
        fn map_keycode(wparam: WPARAM, lparam: LPARAM) -> i32 {
            let scancode = ((lparam & 0x00ff0000) >> 16) as u32;
            let extended = (lparam & 0x01000000) != 0;

            match wparam as i32 {
                VK_SHIFT => unsafe {
                    MapVirtualKeyW(scancode, MAPVK_VSC_TO_VK_EX) as i32
                },
                VK_CONTROL => {
                    if extended { VK_RCONTROL } else { VK_LCONTROL }
                },
                VK_MENU => {
                    if extended { VK_RMENU } else { VK_LMENU }
                },
                _ => wparam as i32
            }
        }
        // Maps the interesting window messages to different Event variants. 
        match message {
            WM_ACTIVATE => send_event(Event::Focused(wparam != 0)),
            WM_DESTROY => send_event(Event::Closed),
            WM_SIZE => send_event(Event::Resized(
                lparam as u16,
                (lparam >> 16) as u16
            )),
            WM_MOVE => send_event(Event::Moved(
                lparam as i16,
                (lparam >> 16) as i16
            )),
            WM_KEYDOWN => send_event(Event::KeyboardInput(
                KeyCode::from_virtual_key(map_keycode(wparam, lparam)),
                if (lparam & (1 << 30)) == 0 {
                    KeyboardEvent::Pressed
                } else {
                    KeyboardEvent::Repeated
                }
            )),
            WM_KEYUP => send_event(Event::KeyboardInput(
                KeyCode::from_virtual_key(map_keycode(wparam, lparam)),
                KeyboardEvent::Released
            )),

            // Mouse events
            WM_MOUSEMOVE => send_mouse_event(MouseEvent::Move, lparam),
            WM_LBUTTONDOWN => send_mouse_event(MouseEvent::LeftButtonDown, lparam),
            WM_LBUTTONUP => send_mouse_event(MouseEvent::LeftButtonUp, lparam),
            WM_RBUTTONDOWN => send_mouse_event(MouseEvent::RightButtonDown, lparam),
            WM_RBUTTONUP => send_mouse_event(MouseEvent::RightButtonUp, lparam),
            WM_MBUTTONDOWN => send_mouse_event(MouseEvent::MiddleButtonDown, lparam),
            WM_MBUTTONUP => send_mouse_event(MouseEvent::MiddleButtonUp, lparam),

            _ => DefWindowProcW(hwnd, message, wparam, lparam)       
        }
        

    }

    impl WindowTrait for Win32Window {
        fn open(x_size: i32, y_size: i32) -> Win32Window {
            // A channel to send the Window from the thread that opened it, 
            // so we can return it. 
            let (tx, rx) = channel();
            thread::spawn(move || {
                let window = unsafe { create_window(Some(wnd_callback), x_size, y_size) };
                unsafe { ShowWindow(window, SW_SHOWDEFAULT); }
                // Channel to communicate between the WNDPROC and the Window 
                let (wnd_tx, wnd_rx) = channel();
                let context = WindowContext {
                    hwnd: window,
                    send: wnd_tx
                };

                let w = Win32Window {
                    window: window,
                    x_size: x_size,
                    y_size: y_size,
                    recv: wnd_rx
                };
                
                // Set the context
                CONTEXT.with(|cell| {
                    *cell.borrow_mut() = Some(context);
                });
                // Send the window back to the main thread 
                tx.send(w).unwrap();
                let mut msg = unsafe { mem::uninitialized() };
                // Star the message loop in this thread
                loop {
                    unsafe {
                        while PeekMessageW(&mut msg, ptr::null_mut(), 0, 0, PM_REMOVE) > 0 {
                            TranslateMessage(&msg);
                            DispatchMessageW(&msg);
                        }
                    }
                }
            });

            rx.recv().unwrap()
        }

        fn clear(&mut self) {
            unsafe {
                let dc = GetDC(self.window);
                PatBlt(dc, 0, 0, self.x_size, self.y_size, BLACKNESS);
                ReleaseDC(self.window, dc);
            }
        }

        fn poll_event(&mut self) -> Option<Event> {
            match self.recv.try_recv().ok() {
                Some(ev) => {
                    if let Event::Resized(x, y) = ev {
                        self.x_size = x as i32;
                        self.y_size = y as i32;
                    }
                    Some(ev)
                },
                None => None
            }
        }

        fn events(&mut self) -> EventIter {
            EventIter { window: self }
        }
    }
    
    impl Drop for Win32Window {
        fn drop(&mut self) {
            unsafe { DestroyWindow(self.window); }
        }
    }
}
