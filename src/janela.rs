use crate::maquina::maquina::Maquina;
use eframe::egui;
use rfd::FileDialog;

pub struct Janela {
    maquina: Maquina,
}

impl Default for Janela {
    fn default() -> Self {
        Self {
            maquina: Maquina::new(),
        }
    }
}

impl eframe::App for Janela {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // barra lateral
        egui::SidePanel::left("sidebar")
            .resizable(false)
            .default_width(200.0)
            .show(ctx, |ui| {
                ui.add_space(6.0);
                ui.heading("Registradores");
                ui.separator();
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            // Área principal (fora da sidebar). Cuidado com o espaço horizontal para a sidebar não comer a área
            ui.heading("Área principal");
            ui.separator();

            if ui.button("Carregar programa (hex)").clicked() {
                let arquivo = FileDialog::new()
                    .set_title("Carregar um programa")
                    .add_filter("Arquivo Hex SIC/XE (.hex)", &["hex"])
                    .pick_file();

                if let Some(hex) = arquivo
                    && let Ok(hex) = std::fs::read_to_string(hex)
                {
                    let hex = hex.trim();
                    let mut chars = hex.chars();
                    let mut bytes = Vec::with_capacity(hex.len() / 2);

                    // Converter pares de dígitos hexa para bytes
                    while let (Some(digito1), Some(digito2)) = (chars.next(), chars.next()) {
                        let byte = [digito1, digito2].iter().collect::<String>();
                        if let Ok(byte) = u8::from_str_radix(&byte, 16) {
                            bytes.push(byte);
                        }
                    }

                    // TODO: Mensagem de erro caso o carregamento falhe
                    let _ = self.maquina.carregar(&bytes);
                }
            }
        });
    }
}
