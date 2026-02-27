use std::collections::HashMap;
use anyhow::{Result, anyhow};

pub fn ligar_objetos(
    objetos: Vec<String>,
    endereco_carga: usize,
    relocar_agora: bool,
) -> Result<String> {

    let tabela_global = primeira_passagem(&objetos, endereco_carga)?;
    let resultado = segunda_passagem(
        &objetos,
        &tabela_global,
        endereco_carga,
        relocar_agora,
    )?;

    Ok(resultado)
}


/// PRIMEIRA PASSAGEM

fn primeira_passagem(
    objetos: &Vec<String>,
    endereco_carga: usize,
) -> Result<HashMap<String, usize>> {

    let mut tabela_global = HashMap::new();
    let mut endereco_base = endereco_carga;

    for obj in objetos {
        let mut tamanho_modulo = 0;
        let mut nome_modulo = String::new();

        for linha in obj.lines() {

            if linha.starts_with('H') {
                // Exemplo: HMOD1 00000000000E
                nome_modulo = linha[1..7].trim().to_string();
                let tamanho_hex = &linha[13..19];
                tamanho_modulo = usize::from_str_radix(tamanho_hex, 16)?;
            }

            if linha.starts_with('D') {
                // Exemplo: DUM    000003ZERO  000010
                let mut i = 1;
                while i + 12 <= linha.len() {
                    let simbolo = linha[i..i+6].trim().to_string();
                    let endereco_hex = &linha[i+6..i+12];

                    if !simbolo.is_empty() {
                        let endereco_rel = usize::from_str_radix(endereco_hex, 16)?;
                        let endereco_abs = endereco_base + endereco_rel;

                        if tabela_global.contains_key(&simbolo) {
                            return Err(anyhow!("Símbolo duplicado: {}", simbolo));
                        }

                        tabela_global.insert(simbolo, endereco_abs);
                    }

                    i += 12;
                }
            }
        }

        endereco_base += tamanho_modulo;
    }

    Ok(tabela_global)
}


/// SEGUNDA PASSAGEM


fn segunda_passagem(
    objetos: &Vec<String>,
    tabela_global: &HashMap<String, usize>,
    endereco_carga: usize,
    relocar_agora: bool,
) -> Result<String> {

    let mut resultado = String::new();
    let mut endereco_base = endereco_carga;

    for obj in objetos {

        let mut tamanho_modulo = 0;

        for linha in obj.lines() {

            if linha.starts_with('H') {
                let tamanho_hex = &linha[13..19];
                tamanho_modulo = usize::from_str_radix(tamanho_hex, 16)?;
            }

            if linha.starts_with('T') {
                let endereco_hex = &linha[1..7];
                let resto = &linha[7..];

                let endereco_rel = usize::from_str_radix(endereco_hex, 16)?;

                let endereco_final = if relocar_agora {
                    endereco_base + endereco_rel
                } else {
                    endereco_rel
                };

                resultado.push_str(
                    &format!("T{:06X}{}\n", endereco_final, resto)
                );
            }

            if linha.starts_with('M') {
                // Exemplo: M00000102+UM
                let endereco_rel = usize::from_str_radix(&linha[1..7], 16)?;
                let tamanho = &linha[7..9];
                let operacao = &linha[9..10];
                let simbolo = &linha[10..];

                let endereco_final = if relocar_agora {
                    endereco_base + endereco_rel
                } else {
                    endereco_rel
                };

                if let Some(valor_simbolo) = tabela_global.get(simbolo.trim()) {

                    if relocar_agora {
                        // Aqui seria o ponto onde você alteraria o código já gerado
                        // (implementação completa depende do formato do seu T)
                    }

                } else {
                    return Err(anyhow!("Símbolo indefinido: {}", simbolo));
                }

                // Se for apenas ligador (não relocador), repassa M
                if !relocar_agora {
                    resultado.push_str(&format!(
                        "M{:06X}{}{}{}\n",
                        endereco_final,
                        tamanho,
                        operacao,
                        simbolo
                    ));
                }
            }
        }

        endereco_base += tamanho_modulo;
    }

    Ok(resultado)
}