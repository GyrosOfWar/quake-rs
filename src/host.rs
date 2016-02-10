use sdl2;
use sdl2::video::Window;
use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::keyboard::Keycode;
use sdl2::surface::SurfaceRef;

use timer::Timer;
use framebuffer::Framebuffer;

pub struct Host {
    window: Window,
    event_pump: EventPump,
    timer: Timer,
    framebuffer: Framebuffer
}

impl Host {
    pub fn new(x: u32, y: u32) -> Host {
        let context = sdl2::init().unwrap();
        let video = context.video().unwrap();
        let window = video.window("rsquake", x, y).build().unwrap();
        
        Host {
            window: window,
            event_pump: context.event_pump().unwrap(),
            timer: Timer::new(), 
            framebuffer: Framebuffer::new(x as usize, y as usize)
        }
    }
    
    fn frame(&mut self) {
        if let Some(timestep) = self.timer.step() {
            self.framebuffer.fill(12);
            let colors = self.framebuffer.to_color_buffer();
            let mut surface = self.window.surface_mut(&self.event_pump).unwrap();
            let mut pixels = surface.without_lock_mut().unwrap();
            // TODO put the color bytes into the surface, blit the surface to the screen
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