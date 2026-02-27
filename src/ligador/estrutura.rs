use std::collections::HashMap;

pub struct Modulo {
    pub nome: String,
    pub codigo: Vec<u8>,
    pub tamanho: u32,
    pub definicoes: HashMap<String, u32>,
    pub referencias: Vec<String>,
    pub modificacoes: Vec<Modificacao>,
}

pub struct Modificacao {
    pub endereco: u32,
    pub tamanho: u8,
    pub simbolo: String,
}