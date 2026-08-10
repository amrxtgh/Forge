pub mod event;
pub mod imgui_layer;
pub mod layer;
pub mod logger;
pub mod window;
use crate::event::*;
use crate::layer::Layer;
use crate::layer::LayerStack;
use crate::logger::Logger;
use crate::window::Window;
use crate::window::WindowProps;
use sdl3::event::Event as SdlEvent;

pub struct Application {
    running: bool,
    window: Window,
    layer_stack: LayerStack,
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
            layer_stack: LayerStack::new(),
        }
    }

    pub fn push_layer(&mut self, layer: Box<dyn Layer>) {
        self.layer_stack.push_layer(layer)
    }
    pub fn push_overlay(&mut self, overlay: Box<dyn Layer>) {
        self.layer_stack.push_overlay(overlay);
    }
    pub fn run(&mut self) {
        Logger::init();
        Logger::core_info("Forge Application Loop Started");
        while self.running {
            let mut pending_events = Vec::new();

            let events: Vec<SdlEvent> = self.window.event_pump.poll_iter().collect();
            for sdl_event in events {
                if let Some(forge_event) = Self::map_sdl_event(sdl_event) {
                    pending_events.push(forge_event);
                }
            }
            for event in pending_events {
                self.on_event(event);
            }
            for layer in self.layer_stack.iter_mut() {
                layer.on_update();
            }

            self.window.on_update();
            std::thread::sleep(std::time::Duration::from_millis(16));
        }
        Logger::core_info("Forge Application Loop Terminated Cleanly");
    }
    pub fn on_event(&mut self, event: Event) {
        if let Event::WindowClose = event {
            self.running = false;
        }
        for layer in self.layer_stack.iter_mut() {
            if layer.on_event(&event) {
                Logger::core_warn(&format!("Event blocked by layer: {}", layer.name()));
                break;
            }
        }
    }

    pub fn map_sdl_event(sdl_event: SdlEvent) -> Option<Event> {
        match sdl_event {
            SdlEvent::Quit { .. } => Some(Event::WindowClose),
            SdlEvent::KeyDown {
                keycode: Some(key),
                repeat,
                ..
            } => {
                let forge_key = match key {
                    sdl3::keyboard::Keycode::Escape => KeyCode::Escape,
                    sdl3::keyboard::Keycode::W => KeyCode::W,
                    sdl3::keyboard::Keycode::A => KeyCode::A,
                    sdl3::keyboard::Keycode::S => KeyCode::S,
                    sdl3::keyboard::Keycode::D => KeyCode::D,
                    _ => return None, // Ignore unmapped keys for now
                };
                Some(Event::KeyPressed(KeyPressedEvent {
                    key: forge_key,
                    repeat_count: if repeat { 1 } else { 0 },
                }))
            }
            _ => None,
        }
    }
}
