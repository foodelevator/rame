use super::Layer;
use crate::events::EventListener;
use imgui::{ImGui, ImGuiCond};

pub struct ImGuiLayer {
    imgui: ImGui,
    window_width: u32,
    window_height: u32,
}

impl ImGuiLayer {
    pub fn new() -> Self {
        let imgui = ImGui::init();

        // TODO: FIX

        Self {
            imgui,
            window_width: 0,
            window_height: 0,
        }
    }
}

impl Layer for ImGuiLayer {}

impl EventListener for ImGuiLayer {
    fn on_window_resize(&mut self, width: u32, height: u32) {
        self.window_width = width;
        self.window_height = height;
    }
    fn on_render(&mut self) {
        let ui = self.imgui.frame(imgui::FrameSize::new(self.window_width as _, self.window_height as _, 1.0), 1.0/60.0);
        ui.window(imgui::im_str!("ImGui"))
            .size((300.0, 100.0), ImGuiCond::FirstUseEver)
            .build(|| {
                ui.text(imgui::im_str!("Hello World!"));
                // self.ui.separator();
            });
    }
}
