use egui::{Button, Color32, RichText, Stroke, TextEdit, vec2};

use crate::app::GraphApp;

pub fn render(ctx: &egui::Context, app: &mut GraphApp) {
    let screen_width = ctx.viewport_rect().width();
    let left_width = (screen_width / 3.0) * 0.5;

    egui::SidePanel::left("params")
        .default_width(left_width)
        .resizable(false)
        .show(ctx, |ui| params(ui, app, left_width));

    egui::CentralPanel::default().show(ctx, |ui| graph(ui, app));
}

fn params(ui: &mut egui::Ui, app: &mut GraphApp, screen_width: f32) {
    ui.heading("Function");

    ui.add(
        TextEdit::singleline(&mut app.func)
            .desired_width(screen_width)
            .hint_text("f(x) = x / 2"),
    );

    if ui
        .add_sized(
            vec2(ui.available_width(), 32.0),
            Button::new(RichText::new("Graph").color(Color32::WHITE).size(20.0))
                .fill(Color32::from_rgb(74, 42, 102))
                .stroke(Stroke::new(1.0, Color32::WHITE)),
        )
        .clicked()
    {
        GraphApp::graph_btn_click();
    }

    ui.separator();

    ui.heading("Math Expression");

    let button_size = screen_width / 4.0;

    ui.horizontal(|ui| {
        if ui
            .add_sized(
                vec2(button_size, 20.0),
                Button::new("+").fill(Color32::DARK_GRAY),
            )
            .clicked()
        {}

        if ui
            .add_sized(
                vec2(button_size, 20.0),
                Button::new("-").fill(Color32::DARK_GRAY),
            )
            .clicked()
        {}

        if ui
            .add_sized(
                vec2(button_size, 20.0),
                Button::new("x").fill(Color32::DARK_GRAY),
            )
            .clicked()
        {}

        if ui
            .add_sized(
                vec2(button_size, 20.0),
                Button::new("/").fill(Color32::DARK_GRAY),
            )
            .clicked()
        {}
    });

    ui.horizontal(|ui| {
        if ui
            .add_sized(
                vec2(button_size, 20.0),
                Button::new("^").fill(Color32::DARK_GRAY),
            )
            .clicked()
        {}

        if ui
            .add_sized(
                vec2(button_size, 20.0),
                Button::new("(").fill(Color32::DARK_GRAY),
            )
            .clicked()
        {}

        if ui
            .add_sized(
                vec2(button_size, 20.0),
                Button::new(")").fill(Color32::DARK_GRAY),
            )
            .clicked()
        {}

        if ui
            .add_sized(
                vec2(button_size, 20.0),
                Button::new("C").fill(Color32::DARK_RED),
            )
            .clicked()
        {}
    });
}

fn graph(ui: &mut egui::Ui, app: &mut GraphApp) {
    ui.heading("Graphics");
    ui.label("and more");
}
