#![allow(dead_code)]
#![cfg_attr(feature="nightly", feature(plugin))]
#![cfg_attr(feature="nightly", plugin(clippy))]
#![feature(time2, test)]

extern crate sdl2;
extern crate rand;
extern crate test;
extern crate byteorder;

use host::Host;

mod options;
mod timer;
mod util;
mod host;
mod framebuffer;
mod bezier;
mod lmp;
mod vector;

fn main() {
    Host::new().run();
}
