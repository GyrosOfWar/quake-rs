use timer::Timer;
use sdl2;
use sdl2::video::Window;
use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::keyboard::Keycode;

pub struct Host {
    timer: Timer,
    window: Window,
    event_pump: EventPump
}

impl Host {
    pub fn new(x: u32, y: u32) -> Host {
        let context = sdl2::init().unwrap();
        let video = context.video().unwrap();
        let window = video.window("rsquake", x, y).build().unwrap();
        
        Host { 
            timer: Timer::new(72),
            window: window,
            event_pump: context.event_pump().unwrap()
        }
    }
    
    #[allow(unused_variables)]
    fn frame(&mut self) {
        if let Some(time) = self.timer.filter_time() {
            // TODO advance simulation
            // render graphics

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