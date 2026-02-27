use std::collections::HashMap;

pub struct TabelaGlobal {
    simbolos: HashMap<String, u32>,
}

impl TabelaGlobal {

    pub fn nova() -> Self {
        TabelaGlobal {
            simbolos: HashMap::new(),
        }
    }

    pub fn inserir(&mut self, nome: String, endereco: u32) -> Result<(), String> {
        if self.simbolos.contains_key(&nome) {
            return Err(format!("Erro: símbolo {} já definido", nome));
        }

        self.simbolos.insert(nome, endereco);
        Ok(())
    }

    pub fn buscar(&self, nome: &str) -> Option<u32> {
        self.simbolos.get(nome).copied()
    }
}