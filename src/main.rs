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
    let mut timer = Timer::new();
    let one_sec = Duration::from_millis(1000);
    
    for event in window.events() {
        println!("{:?}", event);
        match event {
            Event::WindowClosed => {
                println!("Window closed!");
                break;
            },
            _ => {}
        }

        
        timer.tock();
        timer.tick();
    }
}