use crate::maquina::maquina::Maquina;
use anyhow::Context;
use rfd::FileDialog;

pub fn carregar_programa(maquina: &mut Maquina) -> anyhow::Result<()> {
    let arquivo = FileDialog::new()
        .set_title("Carregar um programa")
        .add_filter("Arquivo Hex SIC/XE (.hex)", &["hex"])
        .pick_file()
        .context("Arquivo não encontrado")?;

    let hex = std::fs::read_to_string(arquivo).context("Não foi possível ler o arquivo")?;
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

    maquina.carregar(&bytes)
}
