pub trait Window {
    fn open(x_size: i32, y_size: i32) -> Self;
    // TODO more trait methods, like blitting an image to the buffer and so on
}

pub fn open_window<W>(x_size: i32, y_size: i32) -> W where W: Window {
    Window::open(x_size, y_size)
}

// TODO Linux implementation
// TODO open_window function that chooses the right implementation 
// based on operating system

/// Windows stuff
#[cfg(windows)]
pub mod win32 {
    const WIN_CLASS_NAME: &'static str = "rsquake-window";
    
    use super::Window;
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;
    
    use std::ptr;
    use winapi::*;
    use kernel32::*;
    use user32::*;
    
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
        // TODO put stuff in window 
        ShowWindow(main_window, SW_SHOWDEFAULT);
        main_window
    }
    
    pub struct WinApiWindow {
        window: HWND
    }

    unsafe extern "system" fn wnd_callback(hwnd: HWND, uint: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        DefWindowProcW(hwnd, uint, wparam, lparam)
    }
    
    impl Window for WinApiWindow {
        fn open(x_size: i32, y_size: i32) -> WinApiWindow {
            let window = unsafe { create_window(Some(wnd_callback), x_size, y_size) };
            WinApiWindow {
                window: window
            }
        }
    }
}