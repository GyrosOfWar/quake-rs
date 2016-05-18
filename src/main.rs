#![allow(dead_code)]
#![cfg_attr(feature="nightly", feature(plugin, test))]
#![cfg_attr(feature="nightly", plugin(clippy))]

extern crate sdl2;
extern crate rand;
extern crate byteorder;
extern crate hprof;

#[cfg(feature="nightly")]
extern crate test;

use host::Host;

mod drawing;
mod files;
mod util;
mod host;

fn main() {
    Host::new().run();
}
