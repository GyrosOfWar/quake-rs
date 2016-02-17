#![allow(dead_code)]
#![cfg_attr(feature="nightly", feature(plugin))]
#![cfg_attr(feature="nightly", plugin(clippy))]
#![feature(time2)]

extern crate sdl2;
extern crate rand;

use host::Host;

mod options;
mod timer;
mod util;
mod host;
mod framebuffer;
mod vertex;

fn main() {
    Host::new().run();
}
