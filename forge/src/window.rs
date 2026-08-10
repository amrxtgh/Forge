use crate::event::Event;
use sdl3::Sdl;
use sdl3::gpu::{ColorTargetInfo, CommandBuffer, Device};
use sdl3::video::Window as Sdl3Window;

#[derive(Debug, Clone)]
pub struct WindowProps {
    pub title: String,
    pub width: u32,
    pub height: u32,
}
impl Default for WindowProps {
    fn default() -> Self {
        Self {
            title: String::from("FORGE ENGINE"),
            width: 1280,
            height: 720,
        }
    }
}
pub struct Window {
    pub sdl_context: Sdl,
    pub window: Sdl3Window,
    pub gpu_device: Device,
    pub event_pump: sdl3::EventPump,
}

/// Everything a layer needs to draw one GPU frame.
pub struct Frame<'a> {
    pub sdl: &'a mut Sdl,
    pub device: &'a Device,
    pub window: &'a Sdl3Window,
    pub event_pump: &'a sdl3::EventPump,
    pub command_buffer: &'a mut CommandBuffer,
    pub color_targets: &'a [ColorTargetInfo],
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
            .vulkan()
            .resizable()
            .build()
            .expect("Failed to create SDL3 Window");
        let gpu_device = Device::new(sdl3::gpu::ShaderFormat::SPIRV, true)
            .expect("Failed to create SDL3 Gpu Device Context")
            .with_window(&sdl_window)
            .expect("Failed to attach GPU swapchain to window");
        let event_pump = sdl_context.event_pump().expect("Failed to get event pump");

        Self {
            sdl_context,
            window: sdl_window,
            gpu_device,
            event_pump,
        }
    }
    pub fn on_event(&mut self, _event: Event) {}
}