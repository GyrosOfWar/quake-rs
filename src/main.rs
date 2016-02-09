#![allow(dead_code)]
#![cfg_attr(feature="nightly", feature(plugin))]
#![cfg_attr(feature="nightly", plugin(clippy))]
#![feature(time2)]

extern crate winapi;
extern crate kernel32;
extern crate sdl2;
extern crate libc;

use host::Host;
use options::Options;
use std::time::Instant;
use util::DurationExt;

mod options;
mod timer;
mod util;
mod host;

fn main() {
    let options = Options::new();
    let height = options.check_param("-height").unwrap_or(600);
    let width = options.check_param("-width").unwrap_or(800);
    let mut host = Host::new(width, height);
    host.run();
}
