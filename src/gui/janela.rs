use crate::gui::carregar_programa::carregar_programa;
use crate::maquina::maquina::Maquina;
use eframe::egui;

pub struct Janela {
    maquina: Maquina,
    erro: Option<String>,
}

impl Default for Janela {
    fn default() -> Self {
        Self {
            maquina: Maquina::new(),
            erro: None,
        }
    }
}

impl eframe::App for Janela {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Barra lateral
        egui::SidePanel::left("sidebar")
            .resizable(false)
            .default_width(200.0)
            .show(ctx, |ui| {
                ui.add_space(5.0);
                ui.heading("Registradores");
                ui.add_space(1.0);
                ui.separator();
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            // Área principal (fora da sidebar). Cuidado com o espaço horizontal para a sidebar não comer a área
            ui.heading("Área principal");
            ui.separator();

            if ui.button("Carregar programa (hex)").clicked() {
                if let Err(error) = carregar_programa(&mut self.maquina) {
                    self.erro = Some(error.to_string());
                }
            }
        });

        // Janela de erro
        if self.erro.is_some() {
            ctx.show_viewport_immediate(
                egui::ViewportId::from_hash_of("erro"),
                egui::ViewportBuilder::default()
                    .with_title("Erro")
                    .with_inner_size([300., 100.])
                    .with_maximize_button(false)
                    .with_minimize_button(false)
                    .with_resizable(false),
                |ctx, _| {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Focus);
                    egui::CentralPanel::default().show(ctx, |ui| {
                        ui.add_space(ui.available_height() / 8.0);
                        ui.vertical_centered(|ui| {
                            ui.style_mut()
                                .text_styles
                                .insert(egui::TextStyle::Body, egui::FontId::proportional(14.));

                            ui.label(self.erro.as_ref().unwrap_or(&"".to_string()));
                            ui.add_space(5.0);
                            if ui.button("Ok").clicked() {
                                self.erro = None;
                            }
                        });
                    });

                    if ctx.input(|i| i.viewport().close_requested()) {
                        self.erro = None;
                    }
                },
            );
        }
    }
}
