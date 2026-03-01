use crate::carregador::carregador;
use crate::ligador::ligador;
use crate::maquina::maquina::Maquina;
use crate::montador::montador;
use crate::processador_macros::macros;
use anyhow::Context;
use rfd::FileDialog;

pub fn carregar_programa(maquina: &mut Maquina) -> anyhow::Result<usize> {
    let arquivo = FileDialog::new()
        .set_title("Carregar código fonte (.asm)")
        .add_filter("Código SIC/XE (.asm)", &["asm"])
        .pick_file()
        .context("Nenhum arquivo selecionado")?;

    let caminho_arquivo = arquivo.to_str().context("Caminho inválido")?;
    let codigo_expandido = macros::processar(&std::fs::read_to_string(caminho_arquivo)?)?;

    let codigo_objeto = montador::montar(&codigo_expandido)?;
    let (codigo_objeto, endereco_inicial) = ligador::ligar_objeto(&codigo_objeto)?;

    carregador::carregar(maquina, &codigo_objeto, endereco_inicial)?;
    Ok(endereco_inicial)
}
