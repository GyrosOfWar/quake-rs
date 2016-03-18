#![allow(dead_code)]
#![cfg_attr(feature="nightly", feature(plugin))]
#![cfg_attr(feature="nightly", plugin(clippy))]
#![feature(test, copy_from_slice)]

extern crate sdl2;
extern crate rand;
extern crate test;
extern crate byteorder;

use host::Host;

mod drawing;
mod files;
mod util;
mod host;

fn main() {
    Host::new().run();
}
