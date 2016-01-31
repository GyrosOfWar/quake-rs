#![allow(dead_code)]
#![cfg_attr(feature="nightly", feature(plugin))]
#![cfg_attr(feature="nightly", plugin(clippy))]

extern crate winapi;
extern crate kernel32;
extern crate user32;
extern crate gdi32;

use window::{Window, WindowTrait};
use event::*;
use timer::Timer;
use std::{thread, time};
use util::DurationExt;

mod options;
mod window;
mod event;
mod timer;
mod util;

fn main() {
    let mut window = Window::open(800, 600);
    let mut timer = Timer::new();
    let t_wait = time::Duration::from_millis(150);
    'main: loop {
        for event in window.events() {
            match event {
                Event::Closed => break 'main,
                _ => {}
            }
        }
        window.clear();
        thread::sleep(t_wait);
        let t = timer.get_time();
        println!("{:?}", t.millis());
    }
}
