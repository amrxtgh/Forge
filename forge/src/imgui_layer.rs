use crate::layer::Layer;
use crate::{logger::Logger, window::Frame};
use imgui_sdl3::ImGuiSdl3;

pub struct ImGuiLayer {
    imgui: ImGuiSdl3,
}
impl ImGuiLayer {
    pub fn new(device: &sdl3::gpu::Device, window: &sdl3::video::Window) -> Self {
        Self {
            imgui: ImGuiSdl3::new(device, window, |ctx| {
                ctx.set_ini_filename(None);
                ctx.set_log_filename(None);
                ctx.fonts()
                    .add_font(&[imgui::FontSource::DefaultFontData { config: None }]);
            }),
        }
    }
}
impl Layer for ImGuiLayer {
    fn name(&self) -> &str {
        "ImGuiLayerOverlay"
    }
    fn on_system_event(&mut self, event: &sdl3::event::Event) {
        self.imgui.handle_event(event);
    }
    fn on_render(&mut self, frame: &mut Frame) {
        self.imgui.render(
            frame.sdl,
            frame.device,
            frame.window,
            frame.event_pump,
            frame.command_buffer,
            frame.color_targets,
            |ui| {
                ui.window("Forge Engine Diagnostic")
                    .size([300.0, 150.0], imgui::Condition::FirstUseEver)
                    .build(|| {
                        ui.text("Engine State: Active");
                        ui.text("Backend SDL3 + GPU Render Canvas");
                        ui.separator();
                        if ui.button("Trigger Debug Alert") {
                            Logger::core_info("Debug Button Clicked inside the Layer");
                        }
                    });
            },
        );
    }
}

