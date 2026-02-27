use std::collections::HashMap;
use crate::ligador::estrutura::Modulo;

pub fn segunda_passagem(modulos: &mut Vec<Modulo>, tabela: &HashMap<String, u32>) {
    let mut endereco_base = 0;

    for modulo in modulos.iter_mut() {

        for modificacao in &modulo.modificacoes {

            let endereco_real = endereco_base + modificacao.endereco;

            let valor_simbolo = tabela
                .get(&modificacao.simbolo)
                .expect("Símbolo não definido");

            // supondo modificação de 4 bytes
            let mut valor = 0u32;

            for i in 0..4 {
                valor <<= 8;
                valor |= modulo.codigo[(modificacao.endereco as usize) + i] as u32;
            }

            valor += valor_simbolo;

            for i in (0..4).rev() {
                modulo.codigo[(modificacao.endereco as usize) + i] = (valor & 0xFF) as u8;
                valor >>= 8;
            }
        }

        endereco_base += modulo.tamanho;
    }
}