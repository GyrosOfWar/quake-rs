// Export the right window implementation based on which OS we're running.
#[cfg(target_os="windows")] pub use self::win32::Win32Window as Window;
#[cfg(any(target_os="unix", target_os="linux"))] pub use self::nix::X11Window as Window;

#[derive(Debug, PartialEq, Eq)]
pub enum Event {
    // TODO
    Nothing,
    WindowClosed
}

pub struct EventIter<'a> {
    window: &'a Window
}

impl<'a> Iterator for EventIter<'a> {
    type Item = Event;

    fn next(&mut self) -> Option<Event> {
        match self.window.poll_event() {
            Some(ev) => if ev == Event::WindowClosed {
                None
            } else {
                Some(ev)
            },
            None => Some(Event::Nothing)
        }
    }
}

pub trait WindowTrait {
    /// Creates the Window object and opens it with the specified size.
    fn open(x_size: i32, y_size: i32) -> Self;
    /// Clears the window, painting it black. (No colors anymore, I want them to turn black)
    fn clear(&mut self);
    /// Returns an event, if one is available.
    fn poll_event(&self) -> Option<Event>;
    // TODO it should be no problem to implement this in terms of
    // poll_event but I haven't figured out how to just yet
    /// Returns an iterator over the events for the window.
    fn events<'a>(&'a self) -> EventIter<'a>;
}

#[cfg(any(target_os="unix", target_os="linux"))]
mod nix {
    use super::{WindowTrait, Event};

    // TODO implement X11 window or something of the sort
    pub struct X11Window;

    impl WindowTrait for X11Window {
        fn open(x_size: i32, y_size: i32) -> X11Window {
            unimplemented!()
        }

        fn clear(&mut self) {
            unimplemented!()
        }

        fn poll_event(&self) -> Option<Event> {
            unimplemented!()
        }

        fn events<'a>(&'a self) -> EventIter<'a> { unimplemented!() }
    }
}

#[cfg(target_os="windows")]
mod win32 {
    use super::{WindowTrait, Event, EventIter};

    const WIN_CLASS_NAME: &'static str = "rsquake-window";
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

    thread_local! {
        pub static CONTEXT: RefCell<Option<WindowContext>> = RefCell::new(None)
    }

    pub struct WindowContext {
        hwnd: HWND,
        send: Sender<Event>
    }

    /// This function converts Rust strings to UTF-16 strings understood by Windows.
    fn to_wchar(string: &str) -> Vec<u16> {
        OsStr::new(string).encode_wide().collect()
    }

    unsafe fn get_instance() -> HINSTANCE {
        let instance = GetModuleHandleW(ptr::null());
        if instance.is_null() {
            panic!("GetModuleHandleW error: {}", GetLastError());
        }

        instance
    }

    // This function does all the nasty Win32 API business to open a window.
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

        let main_window: HWND = CreateWindowExW(
            0,
            class_name,
            class_name,
            style,
            200,
            200,
            width,
            height,
            ptr::null_mut(),
            ptr::null_mut(),
            instance,
            ptr::null_mut()
        );

        main_window
    }

    pub struct Win32Window {
        window: HWND,
        x_size: i32,
        y_size: i32,
        recv: Receiver<Event>
    }

    unsafe impl Send for Win32Window {}

    unsafe extern "system" fn wnd_callback(hwnd: HWND, message: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        fn send_event(event: Event) {
            CONTEXT.with(|cell| {
                if let Some(ref ctx) = *cell.borrow() {
                    ctx.send.send(event).ok();
                }
            })
        }

        let event = translate_message(message);
        send_event(event);

        DefWindowProcW(hwnd, message, wparam, lparam)
    }

    fn translate_message(msg: u32) -> Event {
        // TODO
        println!("msg = {}", msg);
        match msg {
            WM_CLOSE => Event::WindowClosed,
            _ => Event::Nothing
        }
    }

    impl WindowTrait for Win32Window {
        fn open(x_size: i32, y_size: i32) -> Win32Window {
            let (tx, rx) = channel();
            thread::spawn(move || {
                let window = unsafe { create_window(Some(wnd_callback), x_size, y_size) };
                unsafe { ShowWindow(window, SW_SHOWDEFAULT); }
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
                CONTEXT.with(|cell| {
                    *cell.borrow_mut() = Some(context);
                });

                tx.send(w);
                let mut msg = unsafe { mem::uninitialized() };
                loop {
                    unsafe {
                        while PeekMessageW(&mut msg, ptr::null_mut(), 0, 0,
                                           PM_REMOVE) > 0 {
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

        fn poll_event(&self) -> Option<Event> {
            match self.recv.recv().ok() {
                Some(ev) => Some(ev),
                None => Some(Event::Nothing)
            }
        }

        fn events<'a>(&'a self) -> EventIter<'a> {
            EventIter { window: self }
        }
    }
}
