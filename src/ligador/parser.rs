use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

use crate::ligador::estrutura::{Modulo, Modificacao};

pub fn parse_obj(caminho: &str) -> Modulo {
    let arquivo = File::open(caminho).expect("Erro ao abrir .obj");
    let reader = BufReader::new(arquivo);

    let mut nome = String::new();
    let mut tamanho = 0;
    let mut codigo: Vec<u8> = Vec::new();
    let mut definicoes = HashMap::new();
    let mut referencias = Vec::new();
    let mut modificacoes = Vec::new();

    for linha in reader.lines() {
        let linha = linha.unwrap();
        let partes: Vec<&str> = linha.split_whitespace().collect();

        match partes[0] {
            "H" => {
                nome = partes[1].to_string();
                tamanho = u32::from_str_radix(partes[3], 16).unwrap();
            }
            "D" => {
                let simbolo = partes[1].to_string();
                let endereco = u32::from_str_radix(partes[2], 16).unwrap();
                definicoes.insert(simbolo, endereco);
            }
            "R" => {
                referencias.push(partes[1].to_string());
            }
            "T" => {
                for i in 3..partes.len() {
                    let byte = u8::from_str_radix(partes[i], 16).unwrap();
                    codigo.push(byte);
                }
            }
            "M" => {
                let endereco = u32::from_str_radix(partes[1], 16).unwrap();
                let tamanho = u8::from_str_radix(partes[2], 16).unwrap();
                let simbolo = partes[3].to_string();

                modificacoes.push(Modificacao {
                    endereco,
                    tamanho,
                    simbolo,
                });
            }
            _ => {}
        }
    }

    Modulo {
        nome,
        codigo,
        tamanho,
        definicoes,
        referencias,
        modificacoes,
    }
}