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
    pub struct Window {
        _sdl_context: Sdl,
        sdl_window: Sdl3Window,
        pub event_pump: sdl3::EventPump,
    }

    impl Window {
        pub fn new(props: WindowProps) -> Self {
            let sdl_context = sdl3::init().expect("Failed to initialize SDL3");
        }
    }
    //TODO: tomorrow
}
