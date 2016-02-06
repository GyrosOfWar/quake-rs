use timer::Timer;
use window::{Window, WindowTrait};
use event::Event;

pub struct Host {
    timer: Timer,
    window: Window
}

impl Host {
    pub fn new(x: i32, y: i32) -> Host {
        Host { 
            timer: Timer::new(72), 
            window: Window::open(x, y)
        }
    }
    
    fn frame(&mut self) {
        if self.timer.filter_time() {
            // TODO advance simulation
            // render graphics
            
            self.window.clear();
        }
    }
    
    pub fn run(&mut self) {
        'main: loop {
            for event in self.window.events() {
                match event {
                    Event::Closed => break 'main,
                    _ => {}
                }
            }
        }
    }
}