use crate::maquina::executor;

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

    pub fn ler_instrucao(&mut self) -> anyhow::Result<()> {
        executor::decodificar_instrucao(&mut self.registradores, &mut self.memoria)
    }
}
