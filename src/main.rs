mod janela;
mod maquina;

use eframe::egui;
use janela::Janela;

fn main() -> Result<(), eframe::Error> {
    let mut options = eframe::NativeOptions::default();
    options.viewport = egui::ViewportBuilder::default()
        .with_inner_size(egui::vec2(720.0, 420.0))
        .with_min_inner_size(egui::vec2(500.0, 300.0));

    eframe::run_native(
        "Máquina Virtual",
        options,
        Box::new(|_cc| Ok(Box::<Janela>::default())),
    )
}
