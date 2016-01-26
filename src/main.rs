extern crate winapi;
extern crate kernel32;
extern crate user32;
extern crate gdi32;

use window::{Window, WindowTrait, Event};
use timer::Timer;
use std::time::Duration;
use std::thread;

mod options;
mod window;
mod timer;

fn main() {
    let mut window = Window::open(800, 600);
    let mut is_running = true;
    let mut timer = Timer::new();
    let one_sec = Duration::from_millis(1000);
    
    while is_running {
        if let Some(event) = window.poll_event() {
            match event {
                Event::Nothing => {}
            }
        }
        
        thread::sleep(one_sec);
        timer.tock();
        println!("Total time: {} seconds", timer.elapsed_seconds());
        timer.tick();
    }
}
