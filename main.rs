use eframe::egui;
use zh_tw_fonts::add_zh_tw_ui_fonts;
mod zh_tw_fonts;

fn main() -> eframe::Result<()> {
    let mut native_options = eframe::NativeOptions::default();
    native_options.centered = true;

    eframe::run_native(
        "顯示中文",
        native_options,
        Box::new(|cc| Ok(Box::new(MainApp::new(cc)))),
    )
}

#[derive(Default)]
struct MainApp {
}

impl MainApp {
    fn new(cc: &eframe::CreationContext) -> Self {
        add_zh_tw_ui_fonts(&cc.egui_ctx);
        Self {
        }
    }
}

impl eframe::App for MainApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello, 顯示中文");
        });
    }
}
