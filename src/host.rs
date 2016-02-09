use timer::Timer;
use sdl2;
use sdl2::video::Window;
use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::keyboard::Keycode;
use std::time::{Duration, Instant};

const MAX_FRAMERATE: f32 = 72.0;
static FRAME_DURATION: Duration = Duration::new(0, (1.0 / MAX_FRAMERATE) as u32);

pub struct Host {
    // timer: Timer,
    window: Window,
    event_pump: EventPump,
    t_start: Instant,
    t_last_frame: Instant,
    t_total: Duration,
}

impl Host {
    pub fn new(x: u32, y: u32) -> Host {
        let context = sdl2::init().unwrap();
        let video = context.video().unwrap();
        let window = video.window("rsquake", x, y).build().unwrap();
        
        Host {
            window: window,
            event_pump: context.event_pump().unwrap(),
            t_start: Instant::now(),
            t_last_frame: Instant::now(),
            t_total: Duration::from_millis(0)
        }
    }
    
    fn frame(&mut self) {
        let timestep = self.t_last_frame.elapsed();
        self.t_total = self.t_start.elapsed();
        //if timestep 
    }
    
    pub fn run(&mut self) {
        'main: loop {
        //'main: for i in 0..5000000 {
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