mod maquina;
mod janela;
use janela::Janela;

fn main() -> Result<(), eframe::Error> {
    let mut options = eframe::NativeOptions::default();
    options.initial_window_size = Some(egui::vec2(720.0, 420.0));
    
    eframe::run_native(
        "Maquina Virtual",
        options,
        Box::new(|_cc| Box::<Janela>::default()),
    )
}
