pub mod event;
pub mod logger;
pub mod window;
use sdl3::event::Event as SdlEvent;
use window::Window;

use crate::window::WindowProps;

pub struct Application {
    running: bool,
    window: Window,
}
impl Application {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        let window = Window::new(WindowProps {
            title: title.to_string(),
            width,
            height,
        });
        Self {
            running: true,
            window,
        }
    }

    pub fn run(&mut self) {
        while self.running {
            let events: Vec<SdlEvent> = self.window.event_pump.poll_iter().collect();
            for sdl_event in events {
                self.process_sdl_event(sdl_event);
            }

            self.window.on_update();
            std::thread::sleep(std::time::Duration::from_millis(16));
        }
    }
    pub fn process_sdl_event(&mut self, sdl_event: SdlEvent) {
        unimplemented!();
    }
}
