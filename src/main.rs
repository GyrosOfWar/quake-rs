extern crate winapi;
extern crate kernel32;
extern crate user32;
extern crate gdi32;

use window::{Window, WindowTrait};
use event::*;
use timer::Timer;
use std::time::Duration;
use std::{thread, time};

mod options;
mod window;
mod event;
mod timer;

fn main() {
    let mut window = Window::open(800, 600);
    let mut timer = Timer::new();
    let second = time::Duration::from_secs(1);
    'main: loop {
        for event in window.events() {
            match event {
                Event::Closed => break 'main,
                _ => {}
            }
        }
        window.clear();
        
        timer.tock();
        timer.tick();
    }
}
