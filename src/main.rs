mod gui;
mod maquina;

use eframe::egui;
use gui::janela::Janela;

fn main() -> Result<(), eframe::Error> {
    let mut options = eframe::NativeOptions::default();
    options.viewport = egui::ViewportBuilder::default()
        .with_inner_size(egui::vec2(720.0, 420.0))
        .with_min_inner_size(egui::vec2(500.0, 300.0));

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
