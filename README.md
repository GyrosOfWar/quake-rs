# quake-rs
A Rust re-implementation of Handmade Quake. 

## Goals
* Multiplatform: I would love this to run on Windows 7+ and Linux. I don't have a Mac OS device, so I won't be able to test there.
* Idiomatic Rust, preserving the broad strokes of the original implementation. 
* No unsafe code! (since I'm now using SDL2 and the Rust `std::time` API for the timer)

## Building
As far as pre-installed libraries go, quake-rs requires SDL2. For instructions on installation, see the GitHub page 
for [rust-sdl2](https://github.com/AngryLawyer/rust-sdl2). Currently, the project only builds on nightly, since I want to
use `std::time::Instant`, which is still unstable (added recently). I recommend using multirust-rs to manage multiple Rust versions.

To run tests:
```
cargo test
```

To run the application:
```
cargo run [--release]
```

## Contributing
Contributions are very welcome. I'll try to keep up with the progress of the videos on a week-to-week basis, but I can't guarantee
I'll always have enough time. 
