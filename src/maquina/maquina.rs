use crate::maquina::constantes::{opcodes, registradores};
use bitreader::BitReader;

pub struct Maquina {
    registradores: [u64; 10],
}

impl Maquina {
    pub fn new() -> Self {
        Self {
            registradores: [0; 10],
        }
    }

    fn set_registrador(&mut self, numero: u8, valor: u64) {
        if let Some(registrador) = self.registradores.get_mut(numero as usize) {
            *registrador = valor;

            // Garantir que registradores sem ser o F não passarão de 24 bits
            if numero != registradores::F && *registrador > 0xFFFFFF {
                while *registrador > 0xFFFFFF {
                    *registrador -= 0x1000000;
                }
            }

            // Garantir que o registrador F não passará de 48 bits
            if numero == registradores::F && *registrador > 0xFFFFFFFFFFFF {
                *registrador -= 0x1000000000000;
            }
        }
    }

    pub fn decodificar_palavra(&mut self, palavra: &[u8]) {
        let mut palavra = BitReader::new(palavra);
        if let Ok(opcode) = palavra.read_u8(8) {
            match opcode {
                opcodes::CLEAR => {
                    let Ok(registrador) = palavra.read_u8(4) else {
                        return;
                    };

                    self.set_registrador(registrador, 0);
                }

                // TODO: Outras instruções
                _ => (),
            }
        }
    }
}
