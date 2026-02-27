use std::collections::HashMap;
use crate::ligador::estrutura::Modulo;

pub fn primeira_passagem(modulos: &mut Vec<Modulo>) -> HashMap<String, u32> {
    let mut tabela_global = HashMap::new();
    let mut endereco_base = 0;

    for modulo in modulos.iter_mut() {

        for (simbolo, endereco_local) in &modulo.definicoes {
            let endereco_global = endereco_base + endereco_local;
            tabela_global.insert(simbolo.clone(), endereco_global);
        }

        endereco_base += modulo.tamanho;
    }

    tabela_global
}