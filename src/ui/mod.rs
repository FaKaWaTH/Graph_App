use egui::{
    Button, Color32, Context, FontId, Frame, Layout, Margin, RichText, Stroke, TextEdit, Ui, vec2,
};
use egui_plot::{Line, Plot, PlotPoints};

use crate::app::GraphApp;

// Colors
// text: {
//    Headings: (195, 190, 210)
//    labels: (160, 160, 175)
// }
// inputs: {
//    border: (35, 30, 50)
//    bavkground: (28, 25, 40)
// }
// white: (207, 214, 223) para texto
// panel letf: background(18, 18, 25)
// panel right background(10, 10, 15)
// buttons: {
//      text: (195, 185, 200)
//      keyboard: (25, 25, 40)
//      graph: (75, 40, 105)
// }
// graph: {
//      text: 40, 38, 48
// }

pub struct Theme {
    pub left_panel_bg: Color32,
    pub right_panel_bg: Color32,
    pub header: Color32,
    pub button_bg_board: Color32,
    pub label_color: Color32,
    pub label_bg: Color32,
    pub brush_color: Color32,
}

impl Theme {
    pub fn new() -> Self {
        Self {
            left_panel_bg: Color32::from_rgb(18, 18, 25),
            right_panel_bg: Color32::from_rgb(10, 10, 15),
            header: Color32::from_rgb(195, 190, 210),
            button_bg_board: Color32::from_rgb(50, 50, 80),
            label_color: Color32::from_rgb(160, 160, 175),
            label_bg: Color32::from_rgb(28, 25, 40),
            brush_color: Color32::from_rgb(118, 77, 156),
        }
    }

    pub fn apply(&self, ctx: &Context) {
        let mut style = (*ctx.style()).clone();

        style.visuals.override_text_color = Some(self.label_color);

        style.spacing.window_margin = Margin::ZERO;

        ctx.set_style(style);
    }
}

pub fn render(ctx: &egui::Context, app: &mut GraphApp) {
    let screen_width = ctx.viewport_rect().width();
    let left_width = (screen_width / 3.0) * 0.5;

    egui::SidePanel::left("params")
        .frame(
            Frame::NONE
                .fill(app.theme.left_panel_bg)
                .inner_margin(Margin::same(8_i8)),
        )
        .exact_width(left_width)
        .resizable(false)
        .show(ctx, |ui| params(ui, app));

    egui::CentralPanel::default()
        .frame(
            Frame::NONE
                .fill(app.theme.right_panel_bg)
                .inner_margin(Margin::same(12_i8))
                .outer_margin(Margin::ZERO),
        )
        .show(ctx, |ui| graph(ui, app));
}

fn params(ui: &mut Ui, app: &mut GraphApp) {
    let available_width = ui.available_width();

    ui.heading(
        RichText::new("Graph App")
            .color(app.theme.header)
            .size(28_f32),
    );

    ui.heading(
        RichText::new("Function")
            .color(app.theme.header)
            .size(22_f32),
    );

    ui.add_space(10_f32);

    ui.add_sized(
        vec2(available_width, 32_f32),
        TextEdit::singleline(&mut app.func)
            .background_color(app.theme.label_bg)
            .font(FontId::new(24_f32, egui::FontFamily::Proportional))
            .hint_text("f(x) = sin(x)"),
    );

    ui.add_space(10_f32);

    ui.with_layout(Layout::top_down(egui::Align::Center), |ui| {
        if ui
            .add_sized(
                vec2(available_width * 0.8, 32_f32),
                Button::new(RichText::new("Graph").color(Color32::WHITE).size(20.0))
                    .fill(Color32::from_rgb(74, 42, 102))
                    .stroke(Stroke::new(1.0, Color32::WHITE)),
            )
            .clicked()
        {
            GraphApp::graph_btn_click();
        }
    });

    ui.add_space(10_f32);
    ui.separator();

    ui.heading(RichText::new("Math Expression").color(app.theme.header));
    ui.add_space(10_f32);

    let spacing = ui.spacing().item_spacing.x;
    let button_size = (available_width - spacing * 3.0) / 4.0;
    let button_color = app.theme.button_bg_board;

    let btn_row1 = ["+", "-", "x", "/"];
    let btn_row2 = ["^", "(", ")", "C"];

    if let Some(label) = render_button_keypad(ui, &button_size, &button_color, &btn_row1) {
        match label.as_str() {
            "+" => {}
            "-" => {}
            "x" => {}
            "/" => {}
            _ => {}
        }
    }

    ui.add_space(6_f32);

    if let Some(label) = render_button_keypad(ui, &button_size, &button_color, &btn_row2) {
        match label.as_str() {
            "^" => {}
            "(" => {}
            ")" => {}
            "C" => {}
            _ => {}
        }
    }
}

fn render_button_keypad(
    ui: &mut Ui,
    size: &f32,
    color: &Color32,
    content: &[&str],
) -> Option<String> {
    let mut clicked_button = None;

    ui.horizontal(|ui| {
        for label in content {
            if ui
                .add_sized(vec2(*size, 30.0), Button::new(*label).fill(*color))
                .clicked()
            {
                clicked_button = Some(label.to_string());
            }
        }
    });

    clicked_button
}

fn graph(ui: &mut Ui, app: &mut GraphApp) {
    ui.horizontal(|ui| {
        ui.heading(RichText::new("Graphic").color(app.theme.header));

        ui.heading(RichText::new("sin(x)").color(app.theme.header));
    });

    let sin: PlotPoints = (0..64)
        .map(|i| {
            let x = i as f64 * 0.1;

            [x, x.sin()]
        })
        .collect();

    let line = Line::new("sin", sin).color(app.theme.brush_color);

    Plot::new("default").show(ui, |plot_ui| plot_ui.line(line));
}
