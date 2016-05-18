#![allow(dead_code)]
#![cfg_attr(feature="nightly", feature(plugin, test))]
#![cfg_attr(feature="nightly", plugin(clippy))]

extern crate sdl2;
extern crate rand;
#[cfg(feature="nightly")]
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
