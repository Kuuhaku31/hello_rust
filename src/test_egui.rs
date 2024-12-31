// test_egui.rs

use eframe::egui;

pub fn test_egui()
{
    fn app(cc: &eframe::CreationContext<'_>) -> Box<dyn eframe::App>
    {
        return Box::new(MyEguiApp::new(cc));
    }

    // 创建一个 egui 应用

    let app_name: &str = "My egui App";

    let native_options: eframe::NativeOptions = eframe::NativeOptions::default();

    let app_creator = Box::new(|cc: &eframe::CreationContext<'_>| Ok(app(cc)));

    // 运行 egui 应用
    let res: Result<(), eframe::Error> = eframe::run_native(app_name, native_options, app_creator);
    if let Err(e) = res
    {
        eprintln!("Error: {e}");
    }
    else
    {
        println!("egui app exited successfully");
    }
}

#[derive(Default)]
struct MyEguiApp {}

impl MyEguiApp
{
    fn new(_cc: &eframe::CreationContext<'_>) -> Self { return Self::default(); }
}

impl eframe::App for MyEguiApp
{
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame)
    {
        fn graph(ui: &mut egui::Ui)
        {
            ui.heading("Hello World!");
            ui.heading("Hello World!");
        }

        egui::CentralPanel::default().show(ctx, |ui: &mut egui::Ui| graph(ui));
    }
}
