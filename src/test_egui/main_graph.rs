// main_graph.rs

use eframe::egui;

use crate::test_egui::MyEguiApp;

pub fn main_graph(this: &mut MyEguiApp, ctx: &egui::Context, ui: &mut egui::Ui)
{
    // 画布
    ui.heading("Hello From egui!");

    ui.horizontal(|ui: &mut egui::Ui| {
        if ui.button("Dark").clicked()
        {
            ctx.set_visuals(egui::Visuals::dark());
        }
        if ui.button("Light").clicked()
        {
            ctx.set_visuals(egui::Visuals::light());
        }
    });

    ui.label(this.frames.to_string());
}
