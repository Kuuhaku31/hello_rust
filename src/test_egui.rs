// test_egui.rs

use eframe::egui::{self, IconData};
use egui_plot::{Line, Plot, PlotPoints};
use image;
use std::sync::Arc;

fn load_fonts(ctx: &egui::Context)
{
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "my_font".to_owned(),
        egui::FontData::from_static(include_bytes!("../misc/fonts/SmileySans-Oblique.ttf")).into(),
    );
    fonts
        .families
        .get_mut(&egui::FontFamily::Proportional)
        .unwrap()
        .insert(0, "my_font".to_owned());
    fonts
        .families
        .get_mut(&egui::FontFamily::Monospace)
        .unwrap()
        .push("my_font".to_owned());
    ctx.set_fonts(fonts);
}

pub fn test_egui()
{
    let mut native_options = eframe::NativeOptions::default();
    let icon_data = include_bytes!("../misc/resources/icon.png");
    let img = image::load_from_memory_with_format(icon_data, image::ImageFormat::Png).unwrap();
    let rgba_data = img.into_rgba8();
    let (w, h) = (rgba_data.width(), rgba_data.height());
    let raw_data: Vec<u8> = rgba_data.into_raw();
    native_options.viewport.icon = Some(Arc::<IconData>::new(IconData {
        rgba: raw_data,
        width: w,
        height: h,
    }));
    eframe::run_native(
        "外特性计算",
        native_options,
        Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc)))),
    );
}

struct MyEguiApp
{
    P: f64,
    T: f64,
    n: f64,
    image_show: bool,
}
impl MyEguiApp
{
    fn new(cc: &eframe::CreationContext<'_>) -> Self
    {
        load_fonts(&cc.egui_ctx);
        Self {
            P: 80.0,
            T: 200.0,
            n: 12000.0,
            image_show: false,
        }
    }
}
impl eframe::App for MyEguiApp
{
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame)
    {
        egui::SidePanel::left("LeftBar").show(ctx, |ui| {
            ui.add(
                egui::DragValue::new(&mut self.P)
                    .prefix("最大功率：")
                    .suffix("kW")
                    .clamp_range(0.1..=1500.0),
            );
            ui.add(
                egui::DragValue::new(&mut self.T)
                    .prefix("最大扭矩：")
                    .suffix("Nm")
                    .clamp_range(0.1..=3000.0),
            );
            ui.add(
                egui::DragValue::new(&mut self.n)
                    .prefix("最高转速：")
                    .suffix("rpm")
                    .clamp_range(0.1..=25000.0)
                    .speed(200),
            );
            if ui.button("显示曲线").clicked()
            {
                self.image_show = true;
            }
            if ui.button("隐藏曲线").clicked()
            {
                self.image_show = false;
            }
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.image_show == true
            {
                let rated_speed = self.P * 9550.0 / self.T;
                let external_characteristic: PlotPoints = (0..self.n.ceil() as usize)
                    .map(|i| {
                        let x = i as f64;
                        [
                            x,
                            if x < rated_speed
                            {
                                self.T
                            }
                            else
                            {
                                self.P * 9550.0 / i as f64
                            },
                        ]
                    })
                    .collect();
                let line = Line::new(external_characteristic).name("external_characteristic");
                let plot = Plot::new("my_plot");
                plot.include_y(0.0)
                    .set_margin_fraction(egui::vec2(0., 0.1))
                    .x_axis_label("转速(rpm)")
                    .y_axis_label("扭矩(N)")
                    .label_formatter(|name, value| {
                        if !name.is_empty()
                        {
                            format!("转速：{:.*}\n扭矩：{:.*}", 1, value.x, 1, value.y)
                        }
                        else
                        {
                            "".to_owned()
                        }
                    })
                    .show(ui, |plot_ui| plot_ui.line(line));
            }
        });
    }
}
