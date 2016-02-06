#![allow(dead_code)]
#![cfg_attr(feature="nightly", feature(plugin))]
#![cfg_attr(feature="nightly", plugin(clippy))]

extern crate winapi;
extern crate kernel32;
extern crate user32;
extern crate gdi32;

use host::Host;
use options::Options;

mod options;
mod window;
mod event;
mod timer;
mod util;
mod host;

fn main() {
    let options = Options::new();
    let mut host = Host::new(800, 600);
    host.run();
}
