use crate::event::Event;
use sdl3::Sdl;
use sdl3::video::Window as Sdl3Window;

pub struct WindowProps {
    pub title: String,
    pub width: u32,
    pub height: u32,
}
impl Default for WindowProps {
    fn default() -> Self {
        Self {
            title: String::from("Forge Engine"),
            width: 1280,
            height: 720,
        }
    }
}
pub struct Window {
    _sdl_context: Sdl,
    sdl_window: Sdl3Window,
    pub event_pump: sdl3::EventPump,
}

impl Window {
    pub fn new(props: WindowProps) -> Self {
        let sdl_context = sdl3::init().expect("Failed to initialize SDL3");
        let video_subsystem = sdl_context.video().expect("Failed to initialize video");
        let sdl_window = video_subsystem
            .window(&props.title, props.width, props.height)
            .position_centered()
            .resizable()
            .build()
            .expect("Failed to create SDL3 Window");

        let event_pump = sdl_context.event_pump().expect("Failed to get event pump");
        Self {
            _sdl_context: sdl_context,
            sdl_window,
            event_pump,
        }
    }
    pub fn on_update(&mut self) {
        unimplemented!();
    }
    pub fn on_event(&mut self, event: Event) {
        unimplemented!();
    }
}
