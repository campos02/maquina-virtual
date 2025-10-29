use crate::maquina::constantes::{opcodes, registradores};
use anyhow::Context;
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

    pub fn registrador(&self, numero: u8) -> Option<u64> {
        self.registradores.get(numero as usize).copied()
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

    pub fn decodificar_instrucao(&mut self, instrucao: &[u8]) -> anyhow::Result<()> {
        let mut palavra = BitReader::new(instrucao);
        if let Ok(opcode) = palavra.read_u8(8) {
            match opcode {
                opcodes::ADDR => {
                    let registrador1 = palavra.read_u8(4).context("Erro ao ler instrução")?;
                    let registrador1 = self
                        .registradores
                        .get(registrador1 as usize)
                        .context("Registrador não encontrado")?;

                    let registrador_destino =
                        palavra.read_u8(4).context("Erro ao ler instrução")?;

                    let registrador2 = self
                        .registradores
                        .get(registrador_destino as usize)
                        .context("Registrador não encontrado")?;

                    self.set_registrador(registrador_destino, registrador1 + registrador2);
                }

                opcodes::CLEAR => {
                    let registrador = palavra.read_u8(4).context("Erro ao ler instrução")?;
                    self.set_registrador(registrador, 0);
                }

                opcodes::COMPR => {
                    let registrador1 = palavra.read_u8(4).context("Erro ao ler instrução")?;
                    let registrador1 = self
                        .registradores
                        .get(registrador1 as usize)
                        .context("Registrador não encontrado")?;

                    let registrador2 = palavra.read_u8(4).context("Erro ao ler instrução")?;
                    let registrador2 = self
                        .registradores
                        .get(registrador2 as usize)
                        .context("Registrador não encontrado")?;

                    let sw = self.registradores[registradores::SW as usize];
                    if registrador1 > registrador2 {
                        // Setar CC para 01
                        self.set_registrador(registradores::SW, sw & 0xFDFFFF);
                        self.set_registrador(registradores::SW, sw | 0x010000);
                    } else if registrador1 < registrador2 {
                        // Setar CC para 11 (-1)
                        self.set_registrador(registradores::SW, sw | 0x030000);
                    } else {
                        // Setar CC para 00
                        self.set_registrador(registradores::SW, sw & 0xFCFFFF);
                    }
                }

                opcodes::DIVR => {
                    let registrador1 = palavra.read_u8(4).context("Erro ao ler instrução")?;
                    let registrador1 = self
                        .registradores
                        .get(registrador1 as usize)
                        .context("Registrador não encontrado")?;

                    let registrador_destino =
                        palavra.read_u8(4).context("Erro ao ler instrução")?;

                    let registrador2 = self
                        .registradores
                        .get(registrador_destino as usize)
                        .context("Registrador não encontrado")?;

                    self.set_registrador(registrador_destino, registrador1 / registrador2);
                }

                opcodes::MULR => {
                    let registrador1 = palavra.read_u8(4).context("Erro ao ler instrução")?;
                    let registrador1 = self
                        .registradores
                        .get(registrador1 as usize)
                        .context("Registrador não encontrado")?;

                    let registrador_destino =
                        palavra.read_u8(4).context("Erro ao ler instrução")?;

                    let registrador2 = self
                        .registradores
                        .get(registrador_destino as usize)
                        .context("Registrador não encontrado")?;

                    self.set_registrador(registrador_destino, registrador1 * registrador2);
                }

                opcodes::RMO => {
                    let registrador1 = palavra.read_u8(4).context("Erro ao ler instrução")?;
                    let registrador1 = self
                        .registradores
                        .get(registrador1 as usize)
                        .context("Registrador não encontrado")?;

                    let registrador_destino =
                        palavra.read_u8(4).context("Erro ao ler instrução")?;

                    self.set_registrador(registrador_destino, *registrador1);
                }

                opcodes::SHIFTL => {
                    let registrador_destino =
                        palavra.read_u8(4).context("Erro ao ler instrução")?;

                    let registrador1 = self
                        .registradores
                        .get(registrador_destino as usize)
                        .context("Registrador não encontrado")?;

                    let bits = palavra.read_u8(4).context("Erro ao ler instrução")?;
                    self.set_registrador(registrador_destino, registrador1 << bits);
                }

                opcodes::SHIFTR => {
                    let registrador_destino =
                        palavra.read_u8(4).context("Erro ao ler instrução")?;

                    let registrador1 = self
                        .registradores
                        .get(registrador_destino as usize)
                        .context("Registrador não encontrado")?;

                    let bits = palavra.read_u8(4).context("Erro ao ler instrução")?;
                    self.set_registrador(registrador_destino, registrador1 >> bits);
                }

                opcodes::SUBR => {
                    let registrador1 = palavra.read_u8(4).context("Erro ao ler instrução")?;
                    let registrador1 = self
                        .registradores
                        .get(registrador1 as usize)
                        .context("Registrador não encontrado")?;

                    let registrador_destino =
                        palavra.read_u8(4).context("Erro ao ler instrução")?;

                    let registrador2 = self
                        .registradores
                        .get(registrador_destino as usize)
                        .context("Registrador não encontrado")?;

                    self.set_registrador(registrador_destino, registrador1 - registrador2);
                }

                opcodes::TIXR => {
                    let x = self.registradores[registradores::X as usize];
                    self.set_registrador(registradores::X, x + 1);

                    let registrador1 = palavra.read_u8(4).context("Erro ao ler instrução")?;
                    let registrador1 = self
                        .registradores
                        .get(registrador1 as usize)
                        .context("Registrador não encontrado")?;

                    let sw = self.registradores[registradores::SW as usize];
                    if x > *registrador1 {
                        // Setar CC para 01
                        self.set_registrador(registradores::SW, sw & 0xFDFFFF);
                        self.set_registrador(registradores::SW, sw | 0x010000);
                    } else if x < *registrador1 {
                        // Setar CC para 11 (-1)
                        self.set_registrador(registradores::SW, sw | 0x030000);
                    } else {
                        // Setar CC para 00
                        self.set_registrador(registradores::SW, sw & 0xFCFFFF);
                    }
                }

                // TODO: Instruções de 3 ou 4 bytes
                _ => (),
            }
        }

        Ok(())
    }
}
