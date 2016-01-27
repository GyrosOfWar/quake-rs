#![feature(log_syntax)]

extern crate winapi;
extern crate kernel32;
extern crate user32;
extern crate gdi32;

use window::{Window, WindowTrait};
use event::*;
use timer::Timer;
use std::time::Duration;
use std::thread;

mod options;
mod window;
mod event;
mod timer;

fn main() {
    let mut window = Window::open(800, 600);
    let mut timer = Timer::new();
    'main: loop {
        for event in window.events() {
            match event {
                Event::Closed => break 'main,
                Event::KeyboardInput(code, KeyboardEvent::Pressed) => {
                    println!("Key pressed: {:?}", code);
                }
                _ => {}
            }
            
            timer.tock();
            timer.tick();
        }   
    }
}
