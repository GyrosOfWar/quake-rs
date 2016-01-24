extern crate winapi;
extern crate kernel32;
extern crate user32;

// use options::Options;
use window::open_window;

mod options;
mod window;

fn main() {
    let window = open_window::<window::win32::WinApiWindow>(800, 600);
    loop {}
}
