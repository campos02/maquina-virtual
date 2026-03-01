use crate::maquina::maquina::Maquina;

pub fn carregar(
    maquina: &mut Maquina,
    objeto_ligado: &str,
    endereco_inicial: usize,
) -> anyhow::Result<()> {
    let mut chars = objeto_ligado.chars();
    let mut bytes = Vec::new();

    while let (Some(d1), Some(d2)) = (chars.next(), chars.next()) {
        let par = format!("{d1}{d2}");
        if let Ok(byte) = u8::from_str_radix(&par, 16) {
            bytes.push(byte);
        }
    }

    maquina.carregar(&bytes, endereco_inicial)
}
