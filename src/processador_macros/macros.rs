use anyhow::anyhow;
use std::collections::HashMap;

#[derive(Default, Clone)]
struct DefinicaoMacro {
    corpo: String,
    parametros: Vec<String>,
}

pub fn processar(entrada: &str) -> anyhow::Result<String> {
    let mut tabela_definicoes = HashMap::new();
    passo(entrada, &mut tabela_definicoes, None)
}

fn passo(
    entrada: &str,
    tabela_definicoes: &mut HashMap<String, DefinicaoMacro>,
    parametros: Option<Vec<(String, String)>>,
) -> anyhow::Result<String> {
    let mut nivel_aninhamento = 0;
    let mut definindo_macro = None;
    let mut saida = String::new();

    for linha in entrada.lines() {
        let mut conteudos = linha.split_whitespace();
        if let Some(mut label) = conteudos.next() {
            // Salvar próximo conteúdo, pois ele pode ser usado depois do if
            let operacao = conteudos.next();

            if let Some(operacao) = operacao {
                if operacao == "MACRO" {
                    nivel_aninhamento += 1;

                    if nivel_aninhamento == 1 {
                        definindo_macro = Some(label);

                        tabela_definicoes.insert(
                            label.to_string(),
                            DefinicaoMacro {
                                parametros: if let Some(parametros) = conteudos.next() {
                                    parametros
                                        .split(',')
                                        .filter(|parametro| {
                                            !parametro.is_empty() && parametro.starts_with('&')
                                        })
                                        .map(|s| s.to_string())
                                        .collect()
                                } else {
                                    Vec::new()
                                },
                                ..Default::default()
                            },
                        );

                        continue;
                    }
                } else if operacao == "MEND" {
                    // Pular labels antes do MEND
                    label = operacao;
                }
            }

            if label == "MEND" {
                nivel_aninhamento -= 1;
                if nivel_aninhamento == 0 {
                    definindo_macro = None;
                    continue;
                }
            }

            // Pular labels antes do macro
            let mut label_pulado = false;
            if !tabela_definicoes.contains_key(label)
                && let Some(operacao) = operacao
            {
                label = operacao;
                label_pulado = true;
            }

            if let Some(nome_macro) = definindo_macro
                && let Some(definicao) = tabela_definicoes.get_mut(nome_macro)
            {
                if let Some(parametros) = &parametros {
                    let mut linha = linha.to_string();

                    // Pular comentários
                    if !linha.starts_with('.') {
                        for (parametro, valor) in parametros {
                            linha = linha.replace(parametro, valor);
                        }

                        // Substituir operador de concatenação
                        linha = linha.replace("->", "");
                    }

                    definicao.corpo.push_str(&linha);
                } else {
                    definicao.corpo.push_str(linha);
                }

                definicao.corpo.push('\n');
            } else if let Some(definicao) = tabela_definicoes.get(label) {
                let parametros = if label_pulado {
                    conteudos.next()
                } else {
                    operacao
                };

                if !definicao.parametros.is_empty()
                    && let Some(parametros) = parametros
                {
                    let parametros = parametros
                        .split(',')
                        .filter(|parametro| !parametro.is_empty())
                        .collect::<Vec<&str>>();

                    if parametros.len() != definicao.parametros.len() {
                        return Err(anyhow!(
                            "Número incorreto de parâmetros\nMacro '{}' espera {} parâmetros, recebeu {}",
                            label,
                            definicao.parametros.len(),
                            parametros.len()
                        ));
                    }

                    // Transformar em um vetor de tuplas com formato (parametro, valor)
                    let parametros = definicao
                        .parametros
                        .iter()
                        .zip(parametros.iter())
                        .map(|(parametro, valor)| (parametro.clone(), valor.to_string()))
                        .collect();

                    // Macros recursivos e expandir macros dentro de macros
                    saida.push_str(&passo(
                        &definicao.corpo.clone(),
                        tabela_definicoes,
                        Some(parametros),
                    )?);
                } else {
                    // Macros recursivos e expandir macros dentro de macros
                    saida.push_str(&passo(&definicao.corpo.clone(), tabela_definicoes, None)?);
                }
            } else {
                if let Some(parametros) = &parametros {
                    let mut linha = linha.to_string();

                    // Pular comentários
                    if !linha.starts_with('.') {
                        for (parametro, valor) in parametros {
                            linha = linha.replace(parametro, valor);
                        }

                        // Substituir operador de concatenação
                        linha = linha.replace("->", "");
                    }

                    saida.push_str(&linha);
                } else {
                    saida.push_str(linha);
                }

                saida.push('\n');
            }
        }
    }

    Ok(saida)
}
