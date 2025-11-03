use crate::maquina::constantes::registradores;
use crate::maquina::executor;
use anyhow::Context;

/// Representa uma máquina SIC/XE.
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

    /// Carrega um programa no endereço 0x6000 da memória.
    pub fn carregar(&mut self, programa: &[u8]) -> anyhow::Result<()> {
        self.memoria
            .get_mut(0x6000..0x6000 + programa.len())
            .context("Programa possui tamanho maior que o possível de carregar")?
            .copy_from_slice(programa);

        executor::set_registrador(&mut self.registradores, registradores::PC, 0x6000);
        Ok(())
    }

    /// Retorna o valor de um registrador caso o número seja válido.
    pub fn registrador(&self, numero: usize) -> Option<u64> {
        self.registradores.get(numero).copied()
    }

    /// Lê da memória, decodifica e executa uma instrução.
    pub fn executar_instrucao(&mut self) -> anyhow::Result<()> {
        executor::executar_instrucao(&mut self.registradores, &mut self.memoria)
    }
}
