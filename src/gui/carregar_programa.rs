use crate::maquina::maquina::Maquina;
use crate::montador::macros;
use crate::montador::montador;
use anyhow::Context;
use rfd::FileDialog;

pub fn carregar_programa(maquina: &mut Maquina) -> anyhow::Result<()> {
    // 1. Abre a janela para selecionar o arquivo .asm
    let arquivo = FileDialog::new()
        .set_title("Carregar código fonte (.asm)")
        .add_filter("Código SIC/XE (.asm)", &["asm"])
        .pick_file()
        .context("Nenhum arquivo selecionado")?;

    let caminho_arquivo = arquivo.to_str().context("Caminho inválido")?;

    // 2. Chama o Processador de Macros (Etapa 3)
    // Isso gera o arquivo MASMAPRG.ASM na pasta do seu projeto
    macros::processar(caminho_arquivo)?;

    // 3. Lê o arquivo expandido gerado pelas macros
    let conteudo_asm = std::fs::read_to_string("MASMAPRG.ASM")
        .context("Erro ao ler MASMAPRG.ASM")?;

    // 4. Roda o Montador (Etapa 2)
    let tabela_simbolos = montador::primeiro_passo(&conteudo_asm)?;
    let registro_objeto = montador::segundo_passo(&conteudo_asm, &tabela_simbolos)?;

    // 5. Converte a string do registro objeto (formato H T E) em bytes reais
    // Vamos focar no registro 'T' (Text) que contém o código
    let bytes = extrair_bytes_do_objeto(&registro_objeto)?;

    // 6. Finalmente carrega na memória da máquina
    maquina.carregar(&bytes)
}

/// Função auxiliar para transformar a string "HTE..." em bytes de verdade
fn extrair_bytes_do_objeto(objeto: &str) -> anyhow::Result<Vec<u8>> {
    let mut bytes = Vec::new();

    // MUDANÇA: Usamos um loop 'for' para ler TODAS as linhas T (Card #49)
    for linha in objeto.lines() {
        if linha.starts_with('T') {
            // No formato SIC/XE: T (1 char) + Endereço (6 chars) + Tamanho (2 chars) = 9 chars de cabeçalho
            // O código real começa no índice 9
            if linha.len() > 9 {
                let codigo_hex = &linha[9..];
                
                let mut chars = codigo_hex.chars();
                while let (Some(d1), Some(d2)) = (chars.next(), chars.next()) {
                    let par = format!("{}{}", d1, d2);
                    if let Ok(byte) = u8::from_str_radix(&par, 16) {
                        bytes.push(byte);
                    }
                }
            }
        }
    }
    
    if bytes.is_empty() {
        return Err(anyhow::anyhow!("Nenhum registro de texto (T) encontrado"));
    }
    
    Ok(bytes)
}