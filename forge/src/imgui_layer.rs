use crate::event::Event;
use crate::layer::Layer;
use imgui::{Context, Ui};
use imgui_sdl3::ImguiSdl3;

pub struct ImGuiLayer {
    imgui_context: Context,
    platform: ImguiSdl3,
}
impl ImGuiLayer {
    pub fn new(sdl_window: &sdl3::video::Window) -> Self {
        let mut imgui_context = Context::create();
        imgui_context.set_ini_filename(None);

        let platform = ImguiSdl3::new(&mut imgui_context, sdl_window);
        Self {
            imgui_context,
            platform,
        }
    }
}
impl Layer for ImGuiLayer {
    fn name(&self) -> &str {
        "ImGuiLayerOverlay"
    }
    fn on_update(&mut self) {
        // Tell backend platform wrapper a new frame is beginning
    }
}
