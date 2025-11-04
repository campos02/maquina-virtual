use crate::maquina::maquina::Maquina;
use anyhow::Context;
use rfd::FileDialog;

pub fn carregar_programa(maquina: &mut Maquina) -> anyhow::Result<()> {
    let arquivo = FileDialog::new()
        .set_title("Carregar programa")
        .add_filter("Arquivo Hex SIC/XE (.hex)", &["hex"])
        .pick_file()
        .context("Nenhum arquivo selecionado")?;

    let hex = std::fs::read_to_string(arquivo).context("Erro ao ler arquivo")?;
    let hex = hex.trim();

    let mut chars = hex.chars();
    let mut bytes = Vec::with_capacity(hex.len() / 2);

    while let (Some(d1), Some(d2)) = (chars.next(), chars.next()) {
        let b = [d1, d2].iter().collect::<String>();
        if let Ok(b) = u8::from_str_radix(&b, 16) {
            bytes.push(b);
        }
    }

    maquina.carregar(&bytes)
}
