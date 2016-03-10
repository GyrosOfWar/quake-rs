#![allow(dead_code)]
#![cfg_attr(feature="nightly", feature(plugin))]
#![cfg_attr(feature="nightly", plugin(clippy))]
#![feature(test)]

extern crate sdl2;
extern crate rand;
extern crate test;
extern crate byteorder;

use host::Host;

// TODO this is getting unwieldy, organize into sub-modules
mod options;
mod timer;
mod util;
mod host;
mod framebuffer;
mod bezier;
mod lmp;
mod vector;
mod files;
mod packfile;

fn main() {
    Host::new().run();
}
