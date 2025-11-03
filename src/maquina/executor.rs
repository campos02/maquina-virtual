use crate::maquina::constantes::{opcodes, registradores};
use anyhow::{Context, anyhow};
use bitreader::BitReader;

/// Seta o valor de um registrador.
/// Necessário usar esta função para eles terem o tamanho correto.
pub fn set_registrador(registradores: &mut [u64; 10], numero: usize, valor: u64) {
    if let Some(registrador) = registradores.get_mut(numero) {
        // Garantir que o F não passará de 48 bits e os outros não passarão de 24 bits
        *registrador = if numero == registradores::F {
            valor % 0x1000000000000
        } else {
            valor % 0x1000000
        };
    }
}

/// Lê da memória, decodifica e executa uma instrução.
pub fn executar_instrucao(
    registradores: &mut [u64; 10],
    memoria: &mut [u8; 32768],
) -> anyhow::Result<()> {
    let Some(proximas) = memoria.get(registradores[registradores::PC] as usize..) else {
        return Err(anyhow!("PC não aponta para um endereço válido"));
    };

    let mut instrucao = BitReader::new(proximas);
    let mut tamanho_instrucao = 2;

    if let Ok(opcode) = instrucao.read_u8(8) {
        match opcode {
            opcodes::ADDR => {
                let registrador1 = instrucao.read_u8(4).context("Erro ao ler instrução")?;
                let registrador1 = registradores
                    .get(registrador1 as usize)
                    .context("Registrador não encontrado")?;

                let registrador_destino = instrucao.read_u8(4).context("Erro ao ler instrução")?;
                let registrador2 = registradores
                    .get(registrador_destino as usize)
                    .context("Registrador não encontrado")?;

                set_registrador(
                    registradores,
                    registrador_destino as usize,
                    registrador1 + registrador2,
                );
            }

            opcodes::CLEAR => {
                let registrador1 = instrucao.read_u8(4).context("Erro ao ler instrução")?;
                set_registrador(registradores, registrador1 as usize, 0);
            }

            opcodes::COMPR => {
                let registrador1 = instrucao.read_u8(4).context("Erro ao ler instrução")?;
                let registrador1 = registradores
                    .get(registrador1 as usize)
                    .context("Registrador não encontrado")?;

                let registrador2 = instrucao.read_u8(4).context("Erro ao ler instrução")?;
                let registrador2 = registradores
                    .get(registrador2 as usize)
                    .context("Registrador não encontrado")?;

                let sw = registradores[registradores::SW];
                if registrador1 > registrador2 {
                    // Setar CC para 01
                    set_registrador(registradores, registradores::SW, sw & 0xFDFFFF);
                    set_registrador(registradores, registradores::SW, sw | 0x010000);
                } else if registrador1 < registrador2 {
                    // Setar CC para 11 (-1)
                    set_registrador(registradores, registradores::SW, sw | 0x018000);
                } else {
                    // Setar CC para 00
                    set_registrador(registradores, registradores::SW, sw & 0xFCFFFF);
                }
            }

            opcodes::DIVR => {
                let registrador1 = instrucao.read_u8(4).context("Erro ao ler instrução")?;
                let registrador1 = registradores
                    .get(registrador1 as usize)
                    .context("Registrador não encontrado")?;

                let registrador_destino = instrucao.read_u8(4).context("Erro ao ler instrução")?;
                let registrador2 = registradores
                    .get(registrador_destino as usize)
                    .context("Registrador não encontrado")?;

                set_registrador(
                    registradores,
                    registrador_destino as usize,
                    registrador1 / registrador2,
                );
            }

            opcodes::MULR => {
                let registrador1 = instrucao.read_u8(4).context("Erro ao ler instrução")?;
                let registrador1 = registradores
                    .get(registrador1 as usize)
                    .context("Registrador não encontrado")?;

                let registrador_destino = instrucao.read_u8(4).context("Erro ao ler instrução")?;
                let registrador2 = registradores
                    .get(registrador_destino as usize)
                    .context("Registrador não encontrado")?;

                set_registrador(
                    registradores,
                    registrador_destino as usize,
                    registrador1 * registrador2,
                );
            }

            opcodes::RMO => {
                let registrador1 = instrucao.read_u8(4).context("Erro ao ler instrução")?;
                let registrador1 = registradores
                    .get(registrador1 as usize)
                    .context("Registrador não encontrado")?;

                let registrador2 = instrucao.read_u8(4).context("Erro ao ler instrução")?;
                set_registrador(registradores, registrador2 as usize, *registrador1);
            }

            opcodes::SHIFTL => {
                let registrador_destino = instrucao.read_u8(4).context("Erro ao ler instrução")?;
                let registrador1 = registradores
                    .get(registrador_destino as usize)
                    .context("Registrador não encontrado")?;

                let bits = instrucao.read_u8(4).context("Erro ao ler instrução")?;
                set_registrador(
                    registradores,
                    registrador_destino as usize,
                    registrador1 << bits,
                );
            }

            opcodes::SHIFTR => {
                let registrador_destino = instrucao.read_u8(4).context("Erro ao ler instrução")?;
                let registrador1 = registradores
                    .get(registrador_destino as usize)
                    .context("Registrador não encontrado")?;

                let bits = instrucao.read_u8(4).context("Erro ao ler instrução")?;
                set_registrador(
                    registradores,
                    registrador_destino as usize,
                    registrador1 >> bits,
                );
            }

            opcodes::SUBR => {
                let registrador1 = instrucao.read_u8(4).context("Erro ao ler instrução")?;
                let registrador1 = registradores
                    .get(registrador1 as usize)
                    .context("Registrador não encontrado")?;

                let registrador_destino = instrucao.read_u8(4).context("Erro ao ler instrução")?;
                let registrador2 = registradores
                    .get(registrador_destino as usize)
                    .context("Registrador não encontrado")?;

                set_registrador(
                    registradores,
                    registrador_destino as usize,
                    registrador1 - registrador2,
                );
            }

            opcodes::TIXR => {
                let x = registradores[registradores::X];
                set_registrador(registradores, registradores::X, x + 1);

                let registrador1 = instrucao.read_u8(4).context("Erro ao ler instrução")?;
                let registrador1 = registradores
                    .get(registrador1 as usize)
                    .context("Registrador não encontrado")?;

                let sw = registradores[registradores::SW];
                if x > *registrador1 {
                    // Setar CC para 01
                    set_registrador(registradores, registradores::SW, sw & 0xFDFFFF);
                    set_registrador(registradores, registradores::SW, sw | 0x010000);
                } else if x < *registrador1 {
                    // Setar CC para 11 (-1)
                    set_registrador(registradores, registradores::SW, sw | 0x018000);
                } else {
                    // Setar CC para 00
                    set_registrador(registradores, registradores::SW, sw & 0xFCFFFF);
                }
            }

            _ => {
                // Últimos 2 bits
                let modo_enderecamento = opcode & 0x03;
                let flags = instrucao
                    .read_u8(if modo_enderecamento == 0 { 1 } else { 4 })
                    .context("Erro ao ler flags da instrução")?;

                // Primeiros 6 bits
                let opcode = opcode & 0xFC;
                let valor = match modo_enderecamento {
                    // Direto formato SIC, verificar somente flag x
                    0 => {
                        let endereco = instrucao
                            .read_u64(15)
                            .context("Erro ao ler valor da instrução")?;

                        let endereco = if flags == 0 {
                            endereco
                        } else {
                            registradores[registradores::X] + endereco
                        };

                        let byte1 = *memoria
                            .get(endereco as usize)
                            .context("Endereço de memória inválido")?;

                        let byte2 = *memoria
                            .get(endereco as usize + 1)
                            .context("Endereço de memória inválido")?;

                        let byte3 = *memoria
                            .get(endereco as usize + 2)
                            .context("Endereço de memória inválido")?;

                        tamanho_instrucao = 3;
                        u64::from_be_bytes([0, 0, 0, 0, 0, byte1, byte2, byte3])
                    }

                    // Imediato
                    1 => match flags {
                        0 => {
                            tamanho_instrucao = 3;
                            instrucao
                                .read_u64(12)
                                .context("Erro ao ler valor da instrução")?
                        }

                        1 => {
                            tamanho_instrucao = 4;
                            instrucao
                                .read_u64(20)
                                .context("Erro ao ler valor da instrução")?
                        }

                        2 => {
                            let valor = instrucao
                                .read_u64(12)
                                .context("Erro ao ler valor da instrução")?;

                            tamanho_instrucao = 3;
                            registradores[registradores::PC] + valor
                        }

                        4 => {
                            let valor = instrucao
                                .read_u64(12)
                                .context("Erro ao ler valor da instrução")?;

                            tamanho_instrucao = 3;
                            registradores[registradores::B] + valor
                        }

                        _ => return Err(anyhow!("Modo de endereçamento inválido")),
                    },

                    // Indireto
                    2 => {
                        let endereco_indireto = match flags {
                            0 => {
                                tamanho_instrucao = 3;
                                instrucao
                                    .read_u64(12)
                                    .context("Erro ao ler endereço da instrução")?
                            }

                            1 => {
                                tamanho_instrucao = 4;
                                instrucao
                                    .read_u64(20)
                                    .context("Erro ao ler endereço da instrução")?
                            }

                            2 => {
                                let endereco = instrucao
                                    .read_u64(12)
                                    .context("Erro ao ler endereço da instrução")?;

                                tamanho_instrucao = 3;
                                registradores[registradores::PC] + endereco
                            }

                            4 => {
                                let endereco = instrucao
                                    .read_u64(12)
                                    .context("Erro ao ler endereço da instrução")?;

                                tamanho_instrucao = 3;
                                registradores[registradores::B] + endereco
                            }

                            _ => return Err(anyhow!("Modo de endereçamento inválido")),
                        };

                        let byte1 = *memoria
                            .get(endereco_indireto as usize)
                            .context("Endereço de memória inválido")?;

                        let byte2 = *memoria
                            .get(endereco_indireto as usize + 1)
                            .context("Endereço de memória inválido")?;

                        let byte3 = *memoria
                            .get(endereco_indireto as usize + 2)
                            .context("Endereço de memória inválido")?;

                        let endereco_dado =
                            u64::from_be_bytes([0, 0, 0, 0, 0, byte1, byte2, byte3]);

                        let byte1 = *memoria
                            .get(endereco_dado as usize)
                            .context("Endereço de memória inválido")?;

                        let byte2 = *memoria
                            .get(endereco_dado as usize + 1)
                            .context("Endereço de memória inválido")?;

                        let byte3 = *memoria
                            .get(endereco_dado as usize + 2)
                            .context("Endereço de memória inválido")?;

                        u64::from_be_bytes([0, 0, 0, 0, 0, byte1, byte2, byte3])
                    }

                    // Direto
                    3 => {
                        let endereco = match flags {
                            0 => {
                                tamanho_instrucao = 3;
                                instrucao
                                    .read_u64(12)
                                    .context("Erro ao ler endereço da instrução")?
                            }

                            1 => {
                                tamanho_instrucao = 4;
                                instrucao
                                    .read_u64(20)
                                    .context("Erro ao ler endereço da instrução")?
                            }

                            2 => {
                                let endereco = instrucao
                                    .read_u64(12)
                                    .context("Erro ao ler endereço da instrução")?;

                                tamanho_instrucao = 3;
                                registradores[registradores::PC] + endereco
                            }

                            4 => {
                                let endereco = instrucao
                                    .read_u64(12)
                                    .context("Erro ao ler endereço da instrução")?;

                                tamanho_instrucao = 3;
                                registradores[registradores::B] + endereco
                            }

                            8 => {
                                let endereco = instrucao
                                    .read_u64(12)
                                    .context("Erro ao ler endereço da instrução")?;

                                tamanho_instrucao = 3;
                                registradores[registradores::X] + endereco
                            }

                            9 => {
                                let endereco = instrucao
                                    .read_u64(20)
                                    .context("Erro ao ler endereço da instrução")?;

                                tamanho_instrucao = 4;
                                registradores[registradores::X] + endereco
                            }

                            10 => {
                                let endereco = instrucao
                                    .read_u64(12)
                                    .context("Erro ao ler endereço da instrução")?;

                                tamanho_instrucao = 3;
                                registradores[registradores::PC]
                                    + registradores[registradores::X]
                                    + endereco
                            }

                            12 => {
                                let endereco = instrucao
                                    .read_u64(12)
                                    .context("Erro ao ler endereço da instrução")?;

                                tamanho_instrucao = 3;
                                registradores[registradores::B]
                                    + registradores[registradores::X]
                                    + endereco
                            }

                            _ => return Err(anyhow!("Modo de endereçamento inválido")),
                        };

                        let byte1 = *memoria
                            .get(endereco as usize)
                            .context("Endereço de memória inválido")?;

                        let byte2 = *memoria
                            .get(endereco as usize + 1)
                            .context("Endereço de memória inválido")?;

                        let byte3 = *memoria
                            .get(endereco as usize + 2)
                            .context("Endereço de memória inválido")?;

                        u64::from_be_bytes([0, 0, 0, 0, 0, byte1, byte2, byte3])
                    }

                    _ => return Err(anyhow!("Modo de endereçamento inválido")),
                };

                match opcode {
                    opcodes::ADD => set_registrador(
                        registradores,
                        registradores::A,
                        registradores[registradores::A] + valor,
                    ),

                    opcodes::AND => set_registrador(
                        registradores,
                        registradores::A,
                        registradores[registradores::A] & valor,
                    ),

                    opcodes::OR => set_registrador(
                        registradores,
                        registradores::A,
                        registradores[registradores::A] | valor,
                    ),

                    opcodes::RSUB => set_registrador(
                        registradores,
                        registradores::PC,
                        registradores[registradores::L],
                    ),

                    opcodes::J => set_registrador(registradores, registradores::PC, valor),

                    opcodes::JEQ => {
                        let cc = registradores[registradores::SW] & 0x018000;
                        if cc == 0 {
                            set_registrador(registradores, registradores::PC, valor);
                        }
                    }

                    opcodes::JGT => {
                        let cc = registradores[registradores::SW] & 0x018000;
                        if cc == 0x010000 {
                            set_registrador(registradores, registradores::PC, valor);
                        }
                    }

                    _ => return Err(anyhow!("Instrução inválida")),
                }
            }
        }
    }

    set_registrador(
        registradores,
        registradores::PC,
        registradores[registradores::PC] + tamanho_instrucao,
    );

    Ok(())
}
