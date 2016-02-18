#![allow(dead_code)]
#![cfg_attr(feature="nightly", feature(plugin))]
#![cfg_attr(feature="nightly", plugin(clippy))]
#![feature(time2, test)]

extern crate sdl2;
extern crate rand;
extern crate test;

use host::Host;

mod options;
mod timer;
mod util;
mod host;
mod framebuffer;
mod bezier;

fn main() {
    Host::new().run();
}
