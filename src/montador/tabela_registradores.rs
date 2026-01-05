use crate::maquina::constantes::registradores;
use phf::phf_map;

pub static TABELA_REGISTRADORES: phf::Map<&'static str, u8> = phf_map! {
    "A" => registradores::A as u8,
    "X" => registradores::X as u8,
    "L" => registradores::L as u8,
    "B" => registradores::B as u8,
    "S" => registradores::S as u8,
    "T" => registradores::T as u8,
    "F" => registradores::F as u8,
    "PC" => registradores::PC as u8,
    "SW" => registradores::SW as u8,
};
