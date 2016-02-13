use sdl2;
use sdl2::video::Window;
use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::keyboard::Keycode;

use timer::Timer;
use options::Options;
use framebuffer::Framebuffer;

use std::ptr;

pub struct Host {
    window: Window,
    event_pump: EventPump,
    timer: Timer,
    framebuffer: Framebuffer,
    options: Options
}

impl Host {
    pub fn new() -> Host {
        let options = Options::new();
        let context = sdl2::init().unwrap();
        let video = context.video().unwrap();
        let width = options.check_param("-width").unwrap_or(800);
        let height = options.check_param("-height").unwrap_or(600);
        let window = video.window("rsquake", width, height).build().unwrap();
        
        Host {
            window: window,
            event_pump: context.event_pump().unwrap(),
            timer: Timer::new(), 
            framebuffer: Framebuffer::new(width as usize, height as usize),
            options: options
        }
    }
    
    fn frame(&mut self) {
        if let Some(_) = self.timer.step() {
            self.framebuffer.fill(8);
            self.framebuffer.line(50, 50, 50, 500, 0);
            {
                let bytes = self.framebuffer.to_bytes();
                let mut surface = self.window.surface_mut(&self.event_pump).unwrap();
                let mut pixels = surface.without_lock_mut().unwrap();
                let src = bytes.as_ptr();
                let dest = pixels.as_mut_ptr();
                unsafe {
                    ptr::copy_nonoverlapping(src, dest, bytes.len());
                }

            }
            self.window.update_surface().unwrap();
        }
    }
    
    pub fn run(&mut self) {
        'main: loop {
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'main
                    },
                    _ => {}
                }
            }
            self.frame();
        }
    }
}