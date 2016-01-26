extern crate winapi;
extern crate kernel32;
extern crate user32;
extern crate gdi32;

// use options::Options;
use window::Window;

mod options;
mod window;

fn main() {
    let mut window = Window::open(800, 600);
    let mut is_running = true;
    while is_running {
        
    }
}
