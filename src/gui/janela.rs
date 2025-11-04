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
        // TOPO (Menu de controle)
        egui::TopBottomPanel::top("barra_superior").show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                ui.heading("🧠 Máquina Virtual SIC/XE");

                ui.separator();

                if ui.button("📂 Carregar programa").clicked() {
                    if let Err(error) = carregar_programa(&mut self.maquina) {
                        self.erro = Some(error.to_string());
                    } else {
                        self.erro = Some("Programa carregado com sucesso.".to_string());
                    }
                }

                if ui.button("▶️ Executar").clicked() {
                    if let Err(error) = self.maquina.executar_instrucao() {
                        self.erro = Some(error.to_string());
                    }
                }

                if ui.button("⏭️ Passo").clicked() {
                    if let Err(error) = self.maquina.executar_instrucao() {
                        self.erro = Some(error.to_string());
                    }
                }

                if ui.button("🔁 Reset").clicked() {
                    self.maquina = Maquina::new();
                    self.erro = None;
                }
            });
        });

        // PAINEL ESQUERDO (Registradores)
        egui::SidePanel::left("painel_registradores")
            .resizable(true)
            .default_width(200.0)
            .show(ctx, |ui| {
                ui.heading("📘 Registradores");
                ui.separator();

                egui::Grid::new("grid_regs").striped(true).show(ui, |ui| {
                    let nomes = ["A", "X", "L", "PC", "SW", "B", "S", "T", "F", "R9"];
                    for (i, nome) in nomes.iter().enumerate() {
                        ui.label(*nome);
                        ui.label(format!("{:06X}", self.maquina.registrador(i).unwrap_or(0)));
                        ui.end_row();
                    }
                });
            });
        // PAINEL CENTRAL (Memória + Código)
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("💾 Memória e Código");
            ui.separator();

            let memoria = self.maquina.memoria();

            egui::ScrollArea::vertical().show(ui, |ui| {
                for addr in (0x6000..0x6020).step_by(8) {
                    let slice = &memoria[addr..addr + 8];
                    ui.monospace(format!(
                        "{:04X}: {:02X} {:02X} {:02X} {:02X}  {:02X} {:02X} {:02X} {:02X}",
                        addr,
                        slice[0], slice[1], slice[2], slice[3],
                        slice[4], slice[5], slice[6], slice[7]
                    ));
                }
            });
        });
        // RODAPÉ (Mensagens)
        egui::TopBottomPanel::bottom("painel_erros")
            .resizable(false)
            .default_height(35.0)
            .show(ctx, |ui| {
                ui.horizontal_centered(|ui| {
                    if let Some(erro) = &self.erro {
                        ui.colored_label(egui::Color32::LIGHT_RED, erro);
                    } else {
                        ui.label("✅ Sistema pronto.");
                    }
                });
            });
    }
}
