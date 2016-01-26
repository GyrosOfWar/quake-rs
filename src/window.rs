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
    
    use std::ptr;
    use winapi::*;
    use kernel32::*;
    use user32::*;
    use gdi32::*;
    
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
        ShowWindow(main_window, SW_SHOWDEFAULT);
        main_window
    }
    
    #[derive(Debug)]
    pub struct Win32Window {
        window: HWND,
        x_size: i32,
        y_size: i32
    }
    
    unsafe extern "system" fn wnd_callback(hwnd: HWND, uint: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        DefWindowProcW(hwnd, uint, wparam, lparam)
    }
    
    fn translate_message(msg: MSG) -> Event {
        // TODO
        match msg.message {
            WM_CLOSE => Event::WindowClosed,
            _ => Event::Nothing
        }
    }
    
    impl WindowTrait for Win32Window {
        fn open(x_size: i32, y_size: i32) -> Win32Window {
            let window = unsafe { create_window(Some(wnd_callback), x_size, y_size) };
            Win32Window {
                window: window,
                x_size: x_size,
                y_size: y_size
            }
        }
       
        fn clear(&mut self) {
            unsafe {
                let dc = GetDC(self.window);
                PatBlt(dc, 0, 0, self.x_size, self.y_size, BLACKNESS);
                ReleaseDC(self.window, dc);
            }
        }
        
        fn poll_event(&self) -> Option<Event> {
            let mut msg = MSG {
                hwnd: ptr::null_mut(),
                message: 0,
                wParam: 0,
                lParam: 0,
                time: 0,
                pt: POINT { x: 0, y: 0}
            };
            let result = unsafe { PeekMessageW(&mut msg, ptr::null_mut(), 0, 0, PM_REMOVE) };
            
            if result != 0 {
                unsafe {
                    TranslateMessage(&mut msg);
                    DispatchMessageW(&mut msg);
                }
                Some(translate_message(msg))
            } else {
                None
            }
        }
        
        fn events<'a>(&'a self) -> EventIter<'a> {
            EventIter { window: self }
        }
    }
}