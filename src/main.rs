mod gui;
mod maquina;

use eframe::egui;
use gui::janela::Janela;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(egui::vec2(1000.0, 600.0))
            .with_min_inner_size(egui::vec2(500.0, 300.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Máquina Virtual",
        options,
        Box::new(|cc| {
            cc.egui_ctx.style_mut(|style| {
                style.spacing.button_padding = egui::vec2(5.0, 5.0);
            });

            Ok(Box::<Janela>::default())
        }),
    )
}
