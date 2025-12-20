use crate::ui::{Theme, render};

pub struct GraphApp {
    pub func: String,
    pub theme: Theme,
}

impl Default for GraphApp {
    fn default() -> Self {
        Self {
            func: String::new(),
            theme: Theme::new(),
        }
    }
}

impl eframe::App for GraphApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        render(ctx, self)
    }
}

impl GraphApp {
    pub fn graph_btn_click() {}
}
