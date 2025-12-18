use crate::ui::render;

pub struct GraphApp {
    pub year: u32,
}

impl Default for GraphApp {
    fn default() -> Self {
        Self { year: 2025 }
    }
}

impl eframe::App for GraphApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        render(ctx, self)
    }
}
