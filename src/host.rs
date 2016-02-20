use sdl2;
use sdl2::video::Window;
use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::keyboard::Keycode;

use timer::Timer;
use options::Options;
use framebuffer::Framebuffer;
use util::DurationExt;
use lmp::LmpImage;
use vector::Vec2;

use std::{ptr, io};
use std::io::prelude::*;

const DEFAULT_WIDTH: u32 = 800;
const DEFAULT_HEIGHT: u32 = 600;

pub struct Host {
    window: Window,
    event_pump: EventPump,
    timer: Timer,
    framebuffer: Framebuffer,
    options: Options,
    debug: bool,
    images: Vec<LmpImage>
}

impl Host {
    pub fn new() -> Host {
        let options = Options::new();
        let context = sdl2::init().unwrap();
        let video = context.video().unwrap();
        let width = options.check_param("-width").unwrap_or(DEFAULT_WIDTH);
        let height = options.check_param("-height").unwrap_or(DEFAULT_HEIGHT);
        let window_builder = video.window("rsquake", width, height);
        let window = window_builder.build().unwrap();
        let debug = options.is_set("-debug");
        // Unlock the framerate in debug mode
        let timer = Timer::new(debug);
        let images = vec![LmpImage::from_file("output.lmp").unwrap()];
        
        Host {
            window: window,
            event_pump: context.event_pump().unwrap(),
            timer: timer,
            framebuffer: Framebuffer::new(width as usize, height as usize),
            options: options,
            debug: debug,
            images: images
        }
    }

    fn frame(&mut self, stdout: &mut io::StdoutLock) {
        if let Some(timestep) = self.timer.step() {
            if self.debug {
                let fps = (1.0 / timestep.seconds()).round();
                write!(stdout, "\r{} FPS", fps).unwrap();
            }

            self.draw();
            self.swap_buffers();
        }
    }

    fn draw(&mut self) {
        self.framebuffer.fill(0);
        self.framebuffer.draw_pic(0, 0, &self.images[0]);
    }

    fn swap_buffers(&mut self) {
        self.framebuffer.swap_buffers();
        {
            let mut surface = self.window.surface_mut(&self.event_pump).unwrap();
            let mut pixels = surface.without_lock_mut().unwrap();
            let bytes = self.framebuffer.color_buffer();
            let src = bytes.as_ptr();
            let dest = pixels.as_mut_ptr();
            unsafe { ptr::copy_nonoverlapping(src, dest, bytes.len()); }
        }
        self.window.update_surface().unwrap();
    }

    pub fn run(&mut self) {
        let stdout = io::stdout();
        let mut lock = stdout.lock();
        if self.debug {
            println!("");
        }
        'main: loop {
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'main
                    },
                    _ => {}
                }
            }
            self.frame(&mut lock);
        }
        if self.debug {
            println!("");
        }
    }
}
