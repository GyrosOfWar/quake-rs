// Export the right window implementation based on which OS we're running.
#[cfg(target_os="windows")] pub use self::win32::Window;
#[cfg(any(target_os="unix", target_os="linux"))] pub use self::nix::Window;

#[cfg(any(target_os="unix", target_os="linux"))] 
mod nix {
    // TODO implement X11 window or something of the sort
    pub struct Window;
    
    impl Window {
        pub fn open(x_size: i32, y_size: i32) -> Window {
            println!("Unix says hi!");
            Window
        }
        
        pub fn clear(&mut self) {
            println!("Clearing window!");
        }
    }
}

#[cfg(target_os="windows")]
mod win32 {
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
    pub struct Window {
        window: HWND,
        x_size: i32,
        y_size: i32
    }
    
    unsafe extern "system" fn wnd_callback(hwnd: HWND, uint: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        DefWindowProcW(hwnd, uint, wparam, lparam)
    }
    
    impl Window {
        /// Creates the window object and opens the window with the specified size.
        pub fn open(x_size: i32, y_size: i32) -> Window {
            let window = unsafe { create_window(Some(wnd_callback), x_size, y_size) };
            Window {
                window: window,
                x_size: x_size,
                y_size: y_size
            }
        }
        
        /// Clears the window, painting it black. (No colors anymore, I want them to turn black)
        pub fn clear(&mut self) {
            unsafe {
                let dc = GetDC(self.window);
                PatBlt(dc, 0, 0, self.x_size, self.y_size, BLACKNESS);
                ReleaseDC(self.window, dc);
            }
        }
    }
}