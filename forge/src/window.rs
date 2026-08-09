use crate::event::Event;
use sdl3::Sdl;
use sdl3::render::Canvas;
use sdl3::video::Window as Sdl3Window;

pub struct WindowProps {
    pub title: String,
    pub width: u32,
    pub height: u32,
}
impl Default for WindowProps {
    fn default() -> Self {
        Self {
            title: String::from("FORGE ENGING"),
            width: 1280,
            height: 720,
        }
    }
}
pub struct Window {
    _sdl_context: Sdl,
    pub canvas: Canvas<Sdl3Window>,
    pub event_pump: sdl3::EventPump,
}

impl Window {
    pub fn new(props: WindowProps) -> Self {
        // initialize platform layers
        let sdl_context = sdl3::init().expect("Failed to initialize SDL3");
        let video_subsystem = sdl_context.video().expect("Failed to initialize video");

        //building the physical os window
        let sdl_window = video_subsystem
            .window(&props.title, props.width, props.height)
            .position_centered()
            .resizable()
            .build()
            .expect("Failed to create SDL3 Window");
        // create the rendering canvas
        let canvas = sdl_window.clone().into_canvas();
        let event_pump = sdl_context.event_pump().expect("Failed to get event pump");

        Self {
            _sdl_context: sdl_context,
            canvas,
            event_pump,
        }
    }
    pub fn on_update(&mut self) {
        self.canvas
            .set_draw_color(sdl3::pixels::Color::RGB(255, 105, 180));
        self.canvas.clear();
        self.canvas.present();
    }
    pub fn on_event(&mut self, _event: Event) {}
}
