use std::collections::HashMap;

pub fn processar(entrada: &str) -> anyhow::Result<String> {
    let mut tabela_definicoes = HashMap::new();
    passo(&entrada, &mut tabela_definicoes)
}

fn passo(entrada: &str, tabela_definicoes: &mut HashMap<String, String>) -> anyhow::Result<String> {
    let mut nivel_aninhamento = 0;
    let mut definindo_macro = None;
    let mut saida = String::new();

    for linha in entrada.lines() {
        let mut argumentos = linha.split_whitespace();
        if let Some(mut label) = argumentos.next() {
            if let Some(operacao) = argumentos.next() {
                if operacao == "MACRO" {
                    nivel_aninhamento += 1;
                    if nivel_aninhamento == 1 {
                        definindo_macro = Some(label);
                        tabela_definicoes.insert(label.to_string(), String::new());
                        continue;
                    }
                } else if operacao == "MEND" {
                    // Pular labels antes do MEND
                    label = operacao;
                }
            }

            if label == "MEND" {
                nivel_aninhamento -= 1;
                if nivel_aninhamento == 0 {
                    definindo_macro = None;
                    continue;
                }
            }

            // Pular labels antes do macro
            if !tabela_definicoes.contains_key(label)
                && let Some(operacao) = argumentos.next()
            {
                label = operacao;
            }

            if let Some(nome_macro) = definindo_macro
                && let Some(corpo) = tabela_definicoes.get_mut(nome_macro)
            {
                corpo.push_str(linha);
                corpo.push_str("\n");
            } else {
                if let Some(corpo) = tabela_definicoes.get(label).cloned() {
                    // Definir macros recursivos e expandir macros dentro de macros
                    saida.push_str(&passo(&corpo, tabela_definicoes)?);
                } else {
                    saida.push_str(linha);
                    saida.push_str("\n");
                }
            }
        }
    }

    Ok(saida)
}
