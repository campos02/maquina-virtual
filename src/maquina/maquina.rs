use crate::maquina::constantes::{opcodes, registradores};
use anyhow::{Context, anyhow};
use bitreader::BitReader;

pub struct Maquina {
    registradores: [u64; 10],
    memoria: [u8; 32768],
}

impl Maquina {
    pub fn new() -> Self {
        Self {
            registradores: [0; 10],
            memoria: [0; 32768],
        }
    }

    pub fn registrador(&self, numero: usize) -> Option<u64> {
        self.registradores.get(numero).copied()
    }

    fn set_registrador(&mut self, numero: usize, valor: u64) {
        if let Some(registrador) = self.registradores.get_mut(numero) {
            // Garantir que o F não passará de 48 bits e os outros não passarão de 24 bits
            *registrador = if numero == registradores::F {
                valor % 0x1000000000000
            } else {
                valor % 0x1000000
            };
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

                    self.set_registrador(registrador_destino as usize, registrador1 + registrador2);
                }

                opcodes::CLEAR => {
                    let registrador = palavra.read_u8(4).context("Erro ao ler instrução")?;
                    self.set_registrador(registrador as usize, 0);
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

                    self.set_registrador(registrador_destino as usize, registrador1 / registrador2);
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

                    self.set_registrador(registrador_destino as usize, registrador1 * registrador2);
                }

                opcodes::RMO => {
                    let registrador1 = palavra.read_u8(4).context("Erro ao ler instrução")?;
                    let registrador1 = self
                        .registradores
                        .get(registrador1 as usize)
                        .context("Registrador não encontrado")?;

                    let registrador_destino =
                        palavra.read_u8(4).context("Erro ao ler instrução")?;

                    self.set_registrador(registrador_destino as usize, *registrador1);
                }

                opcodes::SHIFTL => {
                    let registrador_destino =
                        palavra.read_u8(4).context("Erro ao ler instrução")?;

                    let registrador1 = self
                        .registradores
                        .get(registrador_destino as usize)
                        .context("Registrador não encontrado")?;

                    let bits = palavra.read_u8(4).context("Erro ao ler instrução")?;
                    self.set_registrador(registrador_destino as usize, registrador1 << bits);
                }

                opcodes::SHIFTR => {
                    let registrador_destino =
                        palavra.read_u8(4).context("Erro ao ler instrução")?;

                    let registrador1 = self
                        .registradores
                        .get(registrador_destino as usize)
                        .context("Registrador não encontrado")?;

                    let bits = palavra.read_u8(4).context("Erro ao ler instrução")?;
                    self.set_registrador(registrador_destino as usize, registrador1 >> bits);
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

                    self.set_registrador(registrador_destino as usize, registrador1 - registrador2);
                }

                opcodes::TIXR => {
                    let x = self.registradores[registradores::X];
                    self.set_registrador(registradores::X, x + 1);

                    let registrador1 = palavra.read_u8(4).context("Erro ao ler instrução")?;
                    let registrador1 = self
                        .registradores
                        .get(registrador1 as usize)
                        .context("Registrador não encontrado")?;

                    let sw = self.registradores[registradores::SW];
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

                _ => {
                    // Últimos 2 bits
                    let modo_enderecamento = opcode & 0x03;
                    let flags = palavra
                        .read_u8(4)
                        .context("Erro ao ler flags da instrução")?;

                    // Primeiros 6 bits
                    let opcode = opcode & 0xFC;
                    let valor = match modo_enderecamento {
                        // Direto formato SIC, verificar somente flag x
                        0 => match flags & 8 {
                            0 => {
                                let endereco_base = palavra
                                    .read_u64(12)
                                    .context("Erro ao ler valor da instrução")?;

                                let byte1 = *self
                                    .memoria
                                    .get(endereco_base as usize)
                                    .context("Endereço de memória inválido")?;

                                let byte2 = *self
                                    .memoria
                                    .get(endereco_base as usize + 1)
                                    .context("Endereço de memória inválido")?;

                                let byte3 = *self
                                    .memoria
                                    .get(endereco_base as usize + 2)
                                    .context("Endereço de memória inválido")?;

                                u64::from_be_bytes([0, 0, 0, 0, 0, byte1, byte2, byte3])
                            }

                            8 => {
                                let endereco = palavra
                                    .read_u64(12)
                                    .context("Erro ao ler valor da instrução")?;

                                let endereco_base = self.registradores[registradores::X] + endereco;
                                let byte1 = *self
                                    .memoria
                                    .get(endereco_base as usize)
                                    .context("Endereço de memória inválido")?;

                                let byte2 = *self
                                    .memoria
                                    .get(endereco_base as usize + 1)
                                    .context("Endereço de memória inválido")?;

                                let byte3 = *self
                                    .memoria
                                    .get(endereco_base as usize + 2)
                                    .context("Endereço de memória inválido")?;

                                u64::from_be_bytes([0, 0, 0, 0, 0, byte1, byte2, byte3])
                            }

                            _ => return Err(anyhow!("Modo de endereçamento inválido")),
                        },

                        // Imediato
                        1 => match flags {
                            0 => palavra
                                .read_u64(12)
                                .context("Erro ao ler valor da instrução")?,

                            1 => palavra
                                .read_u64(20)
                                .context("Erro ao ler valor da instrução")?,

                            2 => {
                                let endereco = palavra
                                    .read_u64(12)
                                    .context("Erro ao ler valor da instrução")?;

                                self.registradores[registradores::PC] + endereco
                            }

                            4 => {
                                let endereco = palavra
                                    .read_u64(12)
                                    .context("Erro ao ler valor da instrução")?;

                                self.registradores[registradores::B] + endereco
                            }

                            _ => return Err(anyhow!("Modo de endereçamento inválido")),
                        },

                        // TODO: Direto e indireto
                        _ => return Err(anyhow!("Modo de endereçamento inválido")),
                    };

                    match opcode {
                        opcodes::ADD => self.set_registrador(
                            registradores::A,
                            self.registradores[registradores::A] + valor,
                        ),

                        // TODO: Instruções restantes
                        _ => (),
                    }
                }
            }
        }

        Ok(())
    }
}
