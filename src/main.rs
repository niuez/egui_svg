use eframe;
use egui_extras;
use eframe::egui;
use anyhow::{ anyhow, Context as _ };

fn main() -> anyhow::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([600.0, 800.0]),
        ..Default::default()
    };

    let res = eframe::run_native(
        "Pokemon Viewer",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::<MyApp>::default())
        }),
    ).map_err(|e| anyhow!("failed to run"))?;
    Ok(())
}

#[derive(Default)]
struct MyApp {}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let img_url = format!("file://Lua-Logo.svg");
            egui::ScrollArea::both().show(ui, |ui| {
                ui.image(img_url)
                // ui.add(egui::Image::new(img_url))
            })
        });
    }
}
