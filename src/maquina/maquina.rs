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
        let mut registradores = [0; 10];
        executor::set_registrador(&mut registradores, registradores::PC, 0x6000);

        Self {
            registradores,
            memoria: [0; 32768],
        }
    }

    /// Carrega um programa no endereço 0x6000 da memória.
    pub fn carregar(&mut self, programa: &[u8]) -> anyhow::Result<()> {
        self.memoria[0x6000..].copy_from_slice(&[0; 0x2000]);
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

    /// Retorna um slice da memória.
    pub fn memoria(&self) -> &[u8] {
        &self.memoria
    }

    /// Lê da memória, decodifica e executa uma instrução.
    pub fn executar_instrucao(&mut self) -> anyhow::Result<()> {
        executor::executar_instrucao(&mut self.registradores, &mut self.memoria)
    }

    /// Reseta a máquina sem remover o programa carregado
    pub fn resetar(&mut self) {
        self.memoria[..0x6000].copy_from_slice(&[0; 0x6000]);
        self.registradores = [0; 10];
        executor::set_registrador(&mut self.registradores, registradores::PC, 0x6000);
    }
}
