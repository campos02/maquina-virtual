use crate::montador::tabela_operacoes::{Operacao, TABELA_OPERACOES};
use crate::maquina::constantes::opcodes;
use anyhow::anyhow;
use std::collections::HashMap;

#[============================================================]
#[============================================================]
pub struct MacroDef{
    pub nome: String,
    pub param: Vec<String>,
    #[Param=Parametros]
    pub body: Vec<String>,
}
pub struct MacroProcessador{
    macro_def: HashMap<String,MacroDef>,
}

#[============================================================]
#[============================================================]

impl MacroProcessador {
    pub fn new() -> Self {
        Self {
            MacroDef: HashMap::new()
        }
    }

    #[Processa código assembly]
    pub fn processar(&mut self, entrada: &str) -> Result<String, anyhow::Error> {
        let mut saida = Vec::new();
        let mut linhas = entrada.lines().peekable();
        let mut em_def_macro = false;
        let mut macro_atual: Option<MacroDef> = None;

        #[--------------------------------------------------------]
        #[Coletar definição de macros]
        while let Some(linha) = linhas.next() {
            let trimmed = linha.trim();

            #[pular comentarios e whitespace]
            if trimmed.is_empty() || trimmed.starts_with('.') {
                if em_def_macro {
                    macro_atual.as_mut().unwrap().body.push(linha.to_string());
                } else {
                    saida.push(linha.to_string());
                }
                continue;
            }

            #[--------------------------------------------------------]
            #[Checar inicio de definição de Macro]
            if !em_def_macro {
                let parts: Vec<&str> = trimmed.split_whitespace().collect();

                if parts.len() >= 2 && parts[0] == "MACRO" {
                    #[inicio da definição]
                    em_def_macro = true;
                    let nome_macro = parts[1];
                    let params = if parts.len() > 2 {
                        parts[2..].iter()
                            .map(|s| s.trim_matches(',').to_string())
                            .collect()
                    } else {
                        Vec::new()
                    };

                    macro_atual = Some(MacroDef {
                        nome: nome_macro.to_string(),
                        param: params,
                        body: Vec::new(),
                    });
                    continue;
                }
            } else {
                #[Dentro da definição de Macro_Atual]
                if trimmed == "MEND" {
                    #[final da definição]
                    if let Some(macro_def) = macro_atual.take() {
                        self.macro_def.insert(macro_def.nome.clone(), macro_def);
                    }
                    em_def_macro = false;
                    continue;
                }
                #[Corpo da macro]
                macro_atual.as_mut().unwrap().body.push(linha.to_string());
                continue;
            }
            #[caso n esteja na definição é adicionado a lista]
            saida.push(linha.to_string());
        }
        #[Expansão das macros]
        self.expandir_macros(&mut saida)?;

        Ok(saida.join("\n"))
    }

    #[--------------------------------------------------------]
    #[Expandir chamada de macro]
    fn expandir_macros(&self, saida: &mut Vec<String>) -> Result<(), anyhow::Error> {
        let mut saida_expandida = Vec::new();
        let mut i = 0;

        while i < saida.len() {
            let linha = &saida[i];
            let trimmed = linha.trim();

            if trimmed.is_empty() || trimmed.starts_with('.') {
                saida_expandida.push(linha.clone());
                i += 1;
                continue;
            }

            #[--------------------------------------------------------]

            let parts: Vec<&str> = trimmed.split_whitespace().collect();

            if parts.is_empty() {
                saida_expandida.push(linha.clone());
                i += 1;
                continue;
            }


            #[Checar chamada de macro]
            if let Some(macro_def) = self.macro_def.get(parts[0]) {
                let mut argumentos = Vec::new();

                #[Análise de argumentos]
                if parts.len() > 1 {
                    for arg in &parts[1..] {
                        argumentos.extend(arg.split(',').filter(|s| !s.is_empty()));
                    }
                }

                #[Validação de argumentos]
                if argumentos.len() != macro_def.param.len() {
                    return Err(anyhow!(
                        "Número incorreto de argumentos\nMacro '{}' espera {} argumentos, recebeu {}",
                        macro_def.nome,
                        macro_def.param.len(),
                        argumentos.len()
                    ));
                }

                #[Mapeamento de substituição]
                let substituicoes: HashMap<&str, &str> = macro_def.param
                    .iter()
                    .zip(argumentos.iter())
                    .map(|(param, arg)| (param.as_str(), *arg))
                    .collect();

                #[Expansão de substituição]
                for linha_corpo in &macro_def.body {
                    let mut linha_expandida = linha_corpo.clone();

                    #[Substituição dos parametros]
                    for (param, arg) in &substituicoes {
                        linha_expandida = linha_expandida.replace(&format!("&{}", param), arg);
                    }

                    saida_expandida.push(linha_expandida);
                }
            } else {
                #[Caso não seja chamada de macro]

                saida_expandida.push(linha.clone());
            }

            i += 1;
        }
        *saida = saida_expandida;
        Ok(())
    }


#[--------------------------------------------------------]


    #[limpar definição de macros]
    pub fn limpar(&mut self) {
    self.macro_def.clear();
    }

    #[pegar as deifnições]
        pub fn pegar_nome_macros(&self) -> Vec<&str> {
            self.macro_def.keys().map(|s| s.as_str()).collect()
        }
    }


