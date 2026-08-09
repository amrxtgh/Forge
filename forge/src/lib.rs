pub mod event;
pub mod logger;
pub mod window;
use crate::logger::Logger;
use crate::window::Window;
use sdl3::event::Event as SdlEvent;

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
        Logger::init();
        Logger::core_info("Forge Application Loop Started");
        while self.running {
            let pending_events = Vec::new();

            let events: Vec<SdlEvent> = self.window.event_pump.poll_iter().collect();
            for sdl_event in events {
                if let Some(forge_event) = Self::map_sdl_event(sdl_event) {
                    pending_events.push(forge_event);
                }
            }
            for event in pending_events {
                self.on_event(event);
            }

            self.window.on_update();
            std::thread::sleep(std::time::Duration::from_millis(16));
        }
        Logger::core_info("Forge Application Loop Terminated Cleanly");
    }
    pub fn map_sdl_event(&mut self, sdl_event: SdlEvent) {
        match sdl_event {
            SdlEvent::Quit { .. } => Some(Event::WindowClose),
        }
    }
    pub fn on_event(&mut self; event: Event) {
        unimplemented!();
    }
}
