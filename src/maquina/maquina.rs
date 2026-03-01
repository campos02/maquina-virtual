use crate::maquina::constantes::registradores;
use crate::maquina::executor;
use anyhow::Context;

/// Representa uma máquina SIC/XE.
pub struct Maquina {
    registradores: [u64; 10],
    memoria: [u8; 32768],
    tamanho_programa_atual: usize,
    endereco_inicial_programa: usize,
}

impl Maquina {
    pub fn new() -> Self {
        Self {
            registradores: [0; 10],
            memoria: [0; 32768],
            tamanho_programa_atual: 0,
            endereco_inicial_programa: 0,
        }
    }

    /// Carrega um programa na memória.
    pub fn carregar(&mut self, programa: &[u8], endereco_inicial: usize) -> anyhow::Result<()> {
        self.memoria = [0; 32768];
        self.memoria
            .get_mut(endereco_inicial..endereco_inicial + programa.len())
            .context("Programa possui tamanho maior que o possível de carregar")?
            .copy_from_slice(programa);

        executor::set_registrador(
            &mut self.registradores,
            registradores::PC,
            endereco_inicial as u64,
        );

        self.tamanho_programa_atual = programa.len();
        self.endereco_inicial_programa = endereco_inicial;
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
        if self.tamanho_programa_atual == 0
            || self.registradores[registradores::PC] - self.endereco_inicial_programa as u64
                >= self.tamanho_programa_atual as u64
        {
            Err(anyhow::anyhow!("Execução finalizada"))
        } else {
            executor::executar_instrucao(&mut self.registradores, &mut self.memoria)
        }
    }

    /// Reseta a máquina sem remover o programa carregado
    pub fn resetar(&mut self) {
        if let Some(memoria) = self
            .memoria
            .get_mut(self.endereco_inicial_programa + self.tamanho_programa_atual..)
        {
            for byte in memoria.iter_mut() {
                *byte = 0;
            }
        }

        self.registradores = [0; 10];
        executor::set_registrador(
            &mut self.registradores,
            registradores::PC,
            self.endereco_inicial_programa as u64,
        );
    }
}
