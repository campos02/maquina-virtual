use eframe::egui;

#[derive(Default)]
pub struct Janela {
    //botao_1: u32,
}

impl eframe::App for Janela {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Área principal (fora da sidebar) cuidado com o espaço horizontal para a sidebar n comer a area
            ui.heading("Área principal");
            ui.separator();
            ui.horizontal(|ui|{
                ui.add_space(200.0);
                ui.label("janela pra tralhas.");
            
            })
            
        });

        // barra lateral
        egui::SidePanel::left("sidebar")
            .resizable(false)
            .default_width(200.0)
            .show(ctx, |ui| {
                ui.heading("");
                ui.separator();
                
                // espaçamento
                ui.add_space(20.0);
                
                // The button
                if ui.button("Traduções").clicked() {
                    //implementar tabela de traduções

                }
                
            });
    }
}