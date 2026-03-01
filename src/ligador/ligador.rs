use anyhow::anyhow;
use std::collections::HashMap;
use std::str::from_utf8;

pub fn ligar_objeto(codigo_objeto: &str) -> anyhow::Result<(String, usize)> {
    let tabela = primeira_passagem(codigo_objeto)?;
    segunda_passagem(codigo_objeto, tabela)
}

fn primeira_passagem(codigo_objeto: &str) -> anyhow::Result<HashMap<&str, usize>> {
    let mut localizacao_inicial = 0;
    let mut tamanho_secao_atual = 0;
    let mut tabela_simbolos = HashMap::new();

    for linha in codigo_objeto.lines() {
        match linha.chars().next() {
            Some('H') => {
                if let Some(nome) = linha.get(1..7)
                    && let Some(tamanho) = linha.get(13..19)
                {
                    let nome = nome.trim();
                    tamanho_secao_atual = usize::from_str_radix(tamanho, 16)?;

                    if !tabela_simbolos.contains_key(nome) {
                        tabela_simbolos.insert(nome, localizacao_inicial);
                    } else {
                        return Err(anyhow!("Símbolo {nome} definido múltiplas vezes"));
                    }
                }
            }

            Some('D') => {
                if let Some(linha) = linha.strip_prefix("D") {
                    let mut chunks = linha.as_bytes().chunks(12);

                    while let Some(chunk) = chunks.next()
                        && let Ok(chunk) = from_utf8(chunk)
                        && let Some((simbolo, localizacao)) = chunk.split_at_checked(6)
                    {
                        let simbolo = simbolo.trim();
                        let localizacao = localizacao.trim();

                        if !tabela_simbolos.contains_key(simbolo) {
                            let localizacao =
                                localizacao_inicial + usize::from_str_radix(localizacao, 16)?;

                            tabela_simbolos.insert(simbolo, localizacao);
                        } else {
                            return Err(anyhow!("Símbolo {simbolo} definido múltiplas vezes"));
                        }
                    }
                }
            }

            Some('E') => {
                localizacao_inicial += tamanho_secao_atual;
            }

            _ => (),
        }
    }

    Ok(tabela_simbolos)
}

fn segunda_passagem(
    codigo_objeto: &str,
    tabela_simbolos: HashMap<&str, usize>,
) -> anyhow::Result<(String, usize)> {
    let mut localizacao_inicial = 0;
    let mut tamanho_secao_atual = 0;
    let mut nome_atual = "";
    let mut programa = String::new();
    let mut referencias_atuais = Vec::new();
    let mut inicio_programa = 0;

    let linhas = codigo_objeto.lines().peekable();
    for linha in linhas {
        match linha.chars().next() {
            Some('H') => {
                if let Some(nome) = linha.get(1..7)
                    && let Some(tamanho) = linha.get(13..19)
                {
                    let nome = nome.trim();
                    tamanho_secao_atual = usize::from_str_radix(tamanho, 16)?;
                    nome_atual = nome;
                    referencias_atuais.clear();
                }
            }

            Some('R') => {
                if let Some(linha) = linha.strip_prefix("R") {
                    let mut chunks = linha.as_bytes().chunks(6);

                    while let Some(chunk) = chunks.next()
                        && let Ok(simbolo) = from_utf8(chunk)
                    {
                        let simbolo = simbolo.trim();
                        referencias_atuais.push(simbolo);
                    }
                }
            }

            Some('T') => {
                if let Some(texto) = linha.get(9..) {
                    programa.push_str(texto);
                }
            }

            Some('M') => {
                if let Some(localizacao) = linha.get(1..7)
                    && let Ok(localizacao) = usize::from_str_radix(localizacao.trim(), 16)
                    && let Some(tamanho) = linha.get(7..9)
                    && let Ok(tamanho) = tamanho.parse::<usize>()
                    && let Some(simbolo) = linha.get(9..)
                {
                    let simbolo = simbolo.trim();
                    let (operacao, simbolo) = simbolo.split_at(1);

                    if referencias_atuais.contains(&simbolo)
                        && let Some(localizacao_simbolo) = tabela_simbolos.get(simbolo)
                    {
                        let localizacao = localizacao_inicial + localizacao;
                        if let Some(bytes) = programa.get(localizacao..localizacao + tamanho)
                            && let Ok(bytes) = usize::from_str_radix(bytes, 16)
                        {
                            programa.replace_range(
                                localizacao..localizacao + tamanho,
                                &format!(
                                    "{:X}",
                                    &(if operacao == "-" {
                                        bytes - localizacao_simbolo
                                    } else {
                                        bytes + localizacao_simbolo
                                    })
                                ),
                            );
                        }
                    } else {
                        return Err(anyhow!(
                            "Referência {simbolo} não encontrada na seção {nome_atual}"
                        ));
                    }
                }
            }

            Some('E') => {
                localizacao_inicial += tamanho_secao_atual;
                if let Some(inicio) = linha.get(1..7)
                    && let Ok(inicio) = usize::from_str_radix(inicio, 16)
                {
                    inicio_programa = inicio;
                }
            }

            _ => (),
        }
    }

    Ok((programa, inicio_programa))
}
