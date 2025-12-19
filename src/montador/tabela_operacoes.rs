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
        tamanho: 2,
    },

    "STA" => Operacao::Instrucao {
        hex: opcodes::STA,
        tamanho: 3,
    },

    "+ADD" => Operacao::Instrucao {
        hex: opcodes::ADD,
        tamanho: 4,
    },

    "ADDR" => Operacao::Instrucao {
        hex: opcodes::ADDR,
        tamanho: 2,
    },

    "AND" => Operacao::Instrucao {
        hex: opcodes::AND,
        tamanho: 3,
    },

    "+AND" => Operacao::Instrucao {
        hex: opcodes::AND,
        tamanho: 4,
    },

    "COMP" => Operacao::Instrucao {
        hex: opcodes::COMP,
        tamanho: 3,
    },

    "+COMP" => Operacao::Instrucao {
        hex: opcodes::COMP,
        tamanho: 4,
    },

    "COMPR" => Operacao::Instrucao {
        hex: opcodes::COMPR,
        tamanho: 2,
    },

    "DIV" => Operacao::Instrucao {
        hex: opcodes::DIV,
        tamanho: 3,
    },

    "+DIV" => Operacao::Instrucao {
        hex: opcodes::DIV,
        tamanho: 4,
    },

    "DIVR" => Operacao::Instrucao {
        hex: opcodes::DIVR,
        tamanho: 2,
    },

    "J" => Operacao::Instrucao {
        hex: opcodes::J,
        tamanho: 3,
    },

    "+J" => Operacao::Instrucao {
        hex: opcodes::J,
        tamanho: 4,
    },

    "JEQ" => Operacao::Instrucao {
        hex: opcodes::JEQ,
        tamanho: 3,
    },

    "+JEQ" => Operacao::Instrucao {
        hex: opcodes::JEQ,
        tamanho: 4,
    },

    "JGT" => Operacao::Instrucao {
        hex: opcodes::JGT,
        tamanho: 3,
    },

    "+JGT" => Operacao::Instrucao {
        hex: opcodes::JGT,
        tamanho: 4,
    },

    "JLT" => Operacao::Instrucao {
        hex: opcodes::JLT,
        tamanho: 3,
    },

    "+JLT" => Operacao::Instrucao {
        hex: opcodes::JLT,
        tamanho: 4,
    },

    "JSUB" => Operacao::Instrucao {
        hex: opcodes::JSUB,
        tamanho: 3,
    },

    "+JSUB" => Operacao::Instrucao {
        hex: opcodes::JSUB,
        tamanho: 4,
    },

    "LDA" => Operacao::Instrucao {
        hex: opcodes::LDA,
        tamanho: 3,
    },

    "+LDA" => Operacao::Instrucao {
        hex: opcodes::LDA,
        tamanho: 4,
    },

    "LDB" => Operacao::Instrucao {
        hex: opcodes::LDB,
        tamanho: 3,
    },

    "+LDB" => Operacao::Instrucao {
        hex: opcodes::LDB,
        tamanho: 4,
    },

    "LDCH" => Operacao::Instrucao {
        hex: opcodes::LDCH,
        tamanho: 3,
    },

    "+LDCH" => Operacao::Instrucao {
        hex: opcodes::LDCH,
        tamanho: 4,
    },

    "LDL" => Operacao::Instrucao {
        hex: opcodes::LDL,
        tamanho: 3,
    },

    "+LDL" => Operacao::Instrucao {
        hex: opcodes::LDL,
        tamanho: 4,
    },

    "LDS" => Operacao::Instrucao {
        hex: opcodes::LDS,
        tamanho: 3,
    },

    "+LDS" => Operacao::Instrucao {
        hex: opcodes::LDS,
        tamanho: 4,
    },

    "LDT" => Operacao::Instrucao {
        hex: opcodes::LDT,
        tamanho: 3,
    },

    "+LDT" => Operacao::Instrucao {
        hex: opcodes::LDT,
        tamanho: 4,
    },

    "LDX" => Operacao::Instrucao {
        hex: opcodes::LDX,
        tamanho: 3,
    },

    "+LDX" => Operacao::Instrucao {
        hex: opcodes::LDX,
        tamanho: 4,
    },

    "MUL" => Operacao::Instrucao {
        hex: opcodes::MUL,
        tamanho: 3,
    },

    "+MUL" => Operacao::Instrucao {
        hex: opcodes::MUL,
        tamanho: 4,
    },

    "MULR" => Operacao::Instrucao {
        hex: opcodes::MULR,
        tamanho: 2,
    },

    "OR" => Operacao::Instrucao {
        hex: opcodes::OR,
        tamanho: 3,
    },

    "+OR" => Operacao::Instrucao {
        hex: opcodes::OR,
        tamanho: 4,
    },

    "RMO" => Operacao::Instrucao {
        hex: opcodes::RMO,
        tamanho: 2,
    },

    "RSUB" => Operacao::Instrucao {
        hex: opcodes::RSUB,
        tamanho: 3,
    },

    "+RSUB" => Operacao::Instrucao {
        hex: opcodes::RSUB,
        tamanho: 4,
    },

    "SHIFTL" => Operacao::Instrucao {
        hex: opcodes::SHIFTL,
        tamanho: 2,
    },

    "SHIFTR" => Operacao::Instrucao {
        hex: opcodes::SHIFTR,
        tamanho: 2,
    },

    "+STA" => Operacao::Instrucao {
        hex: opcodes::STA,
        tamanho: 4,
    },

    "STB" => Operacao::Instrucao {
        hex: opcodes::STB,
        tamanho: 3,
    },

    "+STB" => Operacao::Instrucao {
        hex: opcodes::STB,
        tamanho: 4,
    },

    "STCH" => Operacao::Instrucao {
        hex: opcodes::STCH,
        tamanho: 3,
    },

    "+STCH" => Operacao::Instrucao {
        hex: opcodes::STCH,
        tamanho: 4,
    },

    "STL" => Operacao::Instrucao {
        hex: opcodes::STL,
        tamanho: 3,
    },

    "+STL" => Operacao::Instrucao {
        hex: opcodes::STL,
        tamanho: 4,
    },

    "STS" => Operacao::Instrucao {
        hex: opcodes::STS,
        tamanho: 3,
    },

    "+STS" => Operacao::Instrucao {
        hex: opcodes::STS,
        tamanho: 4,
    },

    "STT" => Operacao::Instrucao {
        hex: opcodes::STT,
        tamanho: 3,
    },

    "+STT" => Operacao::Instrucao {
        hex: opcodes::STT,
        tamanho: 4,
    },

    "STX" => Operacao::Instrucao {
        hex: opcodes::STX,
        tamanho: 3,
    },

    "+STX" => Operacao::Instrucao {
        hex: opcodes::STX,
        tamanho: 4,
    },

    "SUB" => Operacao::Instrucao {
        hex: opcodes::SUB,
        tamanho: 3,
    },

    "+SUB" => Operacao::Instrucao {
        hex: opcodes::SUB,
        tamanho: 4,
    },

    "SUBR" => Operacao::Instrucao {
        hex: opcodes::SUBR,
        tamanho: 2,
    },

    "TIX" => Operacao::Instrucao {
        hex: opcodes::TIX,
        tamanho: 3,
    },

    "+TIX" => Operacao::Instrucao {
        hex: opcodes::TIX,
        tamanho: 4,
    },

    "TIXR" => Operacao::Instrucao {
        hex: opcodes::TIXR,
        tamanho: 2,
    }
};
