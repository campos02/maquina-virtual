use crate::maquina::constantes::opcodes;
use crate::montador::tabela_operacoes::{Operacao, TABELA_OPERACOES};
use crate::montador::tabela_registradores::TABELA_REGISTRADORES;
use anyhow::anyhow;
use std::collections::HashMap;

pub fn primeiro_passo(assembly: &str) -> anyhow::Result<HashMap<&str, usize>> {
    // Pular linhas no começo que são só comentários
    let mut linhas = assembly.lines().skip_while(|l| l.trim().starts_with("."));
    let mut contador_localizacao = 0;

    if let Some(linha) = linhas.next() {
        let mut linha = linha.split_whitespace();
        // Pular nome do programa
        linha.next();

        if let Some(operador) = linha.next()
            && operador == "START"
            && let Some(operando) = linha.next()
        {
            contador_localizacao = operando.parse::<usize>().unwrap_or_default();
        }
    }

    let mut tabela_simbolos = HashMap::new();
    for linha in linhas {
        // Remover comentários
        let linha = linha
            .split_once(".")
            .map(|(linha, _)| linha)
            .unwrap_or(linha);

        let Some((label, linha)) = linha.trim().split_once(char::is_whitespace) else {
            continue;
        };

        let operacao_linha;
        let operando;

        if let Some(operacao) = TABELA_OPERACOES.get(label) {
            operacao_linha = operacao;
            operando = linha;
        } else {
            if tabela_simbolos.contains_key(label) {
                return Err(anyhow!("Símbolo {} definido múltiplas vezes", label));
            }

            tabela_simbolos.insert(label, contador_localizacao);
            let Some((operacao, linha)) = linha.trim().split_once(char::is_whitespace) else {
                continue;
            };

            let Some(operacao) = TABELA_OPERACOES.get(operacao) else {
                continue;
            };

            operacao_linha = operacao;
            operando = linha;
        }

        match operacao_linha {
            Operacao::Start => continue,
            Operacao::End => break,

            Operacao::Byte => {
                if let Some(tipo) = operando.get(..1)
                    && let Some(valor) = operando.get(1..)
                {
                    let tamanho = valor.trim_matches('\'').len();
                    if tipo == "C" {
                        contador_localizacao += tamanho;
                    } else if tipo == "X" {
                        contador_localizacao += tamanho.div_ceil(2);
                    }
                }
            }

            Operacao::Word => {
                contador_localizacao += 3;
            }

            Operacao::ReserveWord => {
                contador_localizacao += 3 * operando.parse::<usize>().unwrap_or_default();
            }

            Operacao::ReserveBytes => {
                contador_localizacao += operando.parse::<usize>().unwrap_or_default();
            }

            Operacao::Instrucao { hex: _, tamanho } => {
                contador_localizacao += tamanho;
            }
        }
    }

    Ok(tabela_simbolos)
}

pub fn segundo_passo(
    assembly: &str,
    tabela_simbolos: &HashMap<&str, usize>,
) -> anyhow::Result<String> {
    // Pular linhas no começo que são só comentários
    let mut linhas = assembly.lines().skip_while(|l| l.trim().starts_with("."));

    let mut nome_programa = "";
    let mut endereco_inicial = 0;

    if let Some(linha) = linhas.next() {
        let mut linha = linha.split_whitespace();
        let Some(nome) = linha.next() else {
            return Err(anyhow!("Programa não possui nome"));
        };

        if nome.len() > 6 {
            return Err(anyhow!("Nome do programa tem tamanho maior que 6 bytes"));
        }

        nome_programa = nome;

        if let Some(operador) = linha.next()
            && operador == "START"
            && let Some(operando) = linha.next()
        {
            let operando = usize::from_str_radix(operando, 16).unwrap_or_default();
            endereco_inicial = operando;
        }
    }

    let mut codigo_objeto = String::from("");
    for linha in linhas {
        // Remover comentários
        let linha = linha
            .split_once(".")
            .map(|(linha, _)| linha)
            .unwrap_or(linha);

        let Some((label, linha)) = linha.trim().split_once(char::is_whitespace) else {
            continue;
        };

        let operacao_linha;
        let mut operando;

        if let Some(operacao) = TABELA_OPERACOES.get(label) {
            operacao_linha = operacao;
            operando = linha;
        } else {
            let Some((operacao, linha)) = linha.trim().split_once(char::is_whitespace) else {
                continue;
            };

            let Some(operacao) = TABELA_OPERACOES.get(operacao) else {
                return Err(anyhow!("Operação inválida: {}", operacao));
            };

            operacao_linha = operacao;
            operando = linha;
        }

        match operacao_linha {
            Operacao::End => break,
            Operacao::Instrucao { hex, tamanho } => {
                if *tamanho == 2 {
                    codigo_objeto.push_str(format!("{:02X}", hex).as_str());
                    match *hex {
                        opcodes::CLEAR | opcodes::TIXR => {
                            let r1 = if let Some(r1) = TABELA_REGISTRADORES.get(operando) {
                                *r1
                            } else {
                                let Ok(r1) = operando.parse::<u8>() else {
                                    return Err(anyhow!("Registrador 1 inválido"));
                                };

                                if r1 > 9 {
                                    return Err(anyhow!("Registrador 1 inválido"));
                                }

                                r1
                            };

                            codigo_objeto.push_str(format!("{:X}00", r1).as_str());
                        }

                        _ => {
                            let Some((r1, r2)) = operando.trim().split_once(',') else {
                                return Err(anyhow!("Operando inválido"));
                            };

                            let r1 = if let Some(r1) = TABELA_REGISTRADORES.get(r1) {
                                *r1
                            } else {
                                let Ok(r1) = r1.parse::<u8>() else {
                                    return Err(anyhow!("Registrador 1 inválido"));
                                };

                                if r1 > 9 {
                                    return Err(anyhow!("Registrador 1 inválido"));
                                }

                                r1
                            };

                            let r2 = if let Some(r2) = TABELA_REGISTRADORES.get(r2) {
                                *r2
                            } else {
                                let Ok(r2) = r2.parse::<u8>() else {
                                    return Err(anyhow!("Registrador 2 inválido"));
                                };

                                if r2 > 9 {
                                    return Err(anyhow!("Registrador 2 inválido"));
                                }

                                r2
                            };

                            codigo_objeto.push_str(format!("{:X}{:X}", r1, r2).as_str());
                        }
                    }
                } else {
                    // Modos de endereçamento
                    let enderecamento: u8 = if operando.starts_with("#") {
                        operando = operando.trim_start_matches("#");
                        1
                    } else if operando.starts_with("@") {
                        operando = operando.trim_start_matches("@");
                        2
                    } else {
                        3
                    };

                    codigo_objeto.push_str(format!("{:02X}", hex | enderecamento).as_str());

                    let mut flags_restantes = 0;
                    if *tamanho == 4 {
                        flags_restantes |= 1;
                    }

                    if operando.ends_with(",X") {
                        operando = operando.trim_end_matches(",X");
                        flags_restantes |= 8;
                    }

                    let operando = if let Some(local) = tabela_simbolos.get(operando) {
                        if *local > 4095 && *tamanho < 4 {
                            return Err(anyhow!(
                                "Operando com tamanho maior que o suportado pela instrução"
                            ));
                        }

                        *local
                    } else {
                        let Ok(operando) = operando.parse::<usize>() else {
                            return Err(anyhow!("Operando inválido"));
                        };

                        if operando > 4095 && *tamanho < 4 {
                            return Err(anyhow!(
                                "Operando com tamanho maior que o suportado pela instrução"
                            ));
                        }

                        operando
                    };

                    codigo_objeto.push_str(format!("{:X}", flags_restantes).as_str());
                    if *tamanho < 4 {
                        codigo_objeto.push_str(format!("{:03X}", operando).as_str());
                    } else {
                        codigo_objeto.push_str(format!("{:05X}", operando).as_str());
                    }
                }
            }

            _ => continue,
        }
    }

    Ok(format!(
        "H{nome_programa} {:06X}{:06X}\nT{:06X}{:02X}{codigo_objeto}\nE{:06X}",
        endereco_inicial,
        assembly.len(),
        endereco_inicial,
        codigo_objeto.len(),
        endereco_inicial
    ))
}
