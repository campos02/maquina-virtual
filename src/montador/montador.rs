use crate::montador::tabela_operacoes::{Operacao, TABELA_OPERACOES};
use std::collections::HashMap;

pub fn primeiro_passo(assembly: &str) -> HashMap<&str, usize> {
    // Pular linhas no começo que são só comentários
    let mut linhas = assembly.lines().skip_while(|l| l.trim().starts_with("."));
    let mut contador_localizacao = 0;

    if let Some(linha) = linhas.next() {
        let mut linha = linha.split_whitespace();
        if let Some(operador_ou_label) = linha.next() {
            // Caso for uma label verificar se é seguida de um START
            if operador_ou_label != "START" {
                if let Some(operador) = linha.next()
                    && operador == "START"
                    && let Some(operando) = linha.next()
                {
                    contador_localizacao = operando.parse::<usize>().unwrap_or_default();
                }
            } else if let Some(operando) = linha.next() {
                contador_localizacao = operando.parse::<usize>().unwrap_or_default()
            }
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

        if let Some(operacao) = TABELA_OPERACOES.get(label) {
            match operacao {
                Operacao::Start => continue,
                Operacao::End => break,

                Operacao::Byte => {
                    let operando = linha;
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
                    let operando = linha;
                    contador_localizacao += 3 * operando.parse::<usize>().unwrap_or_default();
                }

                Operacao::ReserveBytes => {
                    let operando = linha;
                    contador_localizacao += operando.parse::<usize>().unwrap_or_default();
                }

                Operacao::Instrucao { hex: _, tamanho } => {
                    contador_localizacao += tamanho;
                }
            }
        } else {
            tabela_simbolos.insert(label, contador_localizacao);
            let Some((operacao, operando)) = linha.trim().split_once(char::is_whitespace) else {
                continue;
            };

            let Some(operacao) = TABELA_OPERACOES.get(operacao) else {
                continue;
            };

            match operacao {
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
                            contador_localizacao += tamanho / 2;
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
    }

    tabela_simbolos
}
