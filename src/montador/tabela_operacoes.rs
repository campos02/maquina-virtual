use crate::maquina::constantes::opcodes;
use phf::phf_map;

pub enum Operacao {
    Start,
    End,
    Byte,
    Word,
    ReserveWord,
    ReserveBytes,
    Instrucao { hex: u8, tamanho: usize },
}

pub static TABELA_OPERACOES: phf::Map<&'static str, Operacao> = phf_map! {
    "START" => Operacao::Start,
    "END" => Operacao::End,
    "BYTE" => Operacao::Byte,
    "WORD" => Operacao::Word,
    "RESW" => Operacao::ReserveWord,
    "RESB" => Operacao::ReserveBytes,

    "ADD" => Operacao::Instrucao {
        hex: opcodes::ADD,
        tamanho: 3,
    },

    "CLEAR" => Operacao::Instrucao {
        hex: opcodes::CLEAR,
        tamanho: 3,
    },

    "STA" => Operacao::Instrucao {
        hex: opcodes::STA,
        tamanho: 3,
    },

    "+ADD" => Operacao::Instrucao {
        hex: opcodes::ADD,
        tamanho: 4,
    }
};
