use std::collections::HashMap;
use std::fs::File;
use std::io::{Write, BufReader, BufRead};

struct MacroDef {
    nome: String,
    instrucoes: Vec<String>,
}

pub fn processar(caminho_entrada: &str) -> anyhow::Result<()> {
    let arquivo = File::open(caminho_entrada)?;
    let leitor = BufReader::new(arquivo);
    
    // 1. Ler todas as linhas do arquivo para um vetor (Input Buffer)
    // Isso nos permite inserir linhas novas no meio do processo
    let mut linhas: Vec<String> = leitor.lines().collect::<Result<_, _>>()?;
    
    let mut namtab: HashMap<String, usize> = HashMap::new();
    let mut deftab: Vec<MacroDef> = Vec::new();
    let mut codigo_expandido = String::new();
    
    let mut definindo_macro: Option<MacroDef> = None;
    let mut nivel_aninhamento = 0;
    
    let mut i = 0;
    while i < linhas.len() {
        let linha = linhas[i].clone();
        let linha_trim = linha.trim();
        
        // Avançamos o índice, mas podemos inserir coisas em 'linhas' depois
        i += 1;

        if linha_trim.is_empty() || linha_trim.starts_with('.') {
            if definindo_macro.is_none() {
                codigo_expandido.push_str(&linha);
                codigo_expandido.push('\n');
            } else if let Some(ref mut m) = definindo_macro {
                m.instrucoes.push(linha);
            }
            continue;
        }

        // 1. Início de Definição (MACRO)
        if linha_trim.contains("MACRO") {
            nivel_aninhamento += 1;
            if nivel_aninhamento == 1 {
                let partes: Vec<&str> = linha_trim.split_whitespace().collect();
                let nome_macro = if partes[0] == "MACRO" { "SEM_NOME" } else { partes[0] };
                definindo_macro = Some(MacroDef {
                    nome: nome_macro.to_string(),
                    instrucoes: Vec::new(),
                });
                continue; // Não escreve a linha 'MACRO' no arquivo final
            }
        }

        // 2. Fim de Definição (MEND)
        if linha_trim == "MEND" {
            nivel_aninhamento -= 1;
            if nivel_aninhamento == 0 {
                if let Some(m) = definindo_macro.take() {
                    namtab.insert(m.nome.clone(), deftab.len());
                    deftab.push(m);
                }
                continue; // Não escreve 'MEND' no arquivo final
            }
        }

        // 3. Se estivermos definindo, guardamos a linha na memória
        if let Some(ref mut m) = definindo_macro {
            m.instrucoes.push(linha);
            continue;
        }

        // 4. Verificação de Chamada (A MÁGICA ACONTECE AQUI)
        let partes: Vec<&str> = linha_trim.split_whitespace().collect();
        let mut macro_encontrada = false;

        for palavra in &partes {
            if let Some(&indice) = namtab.get(*palavra) {
                // Se achou uma macro, NÃO escreve no arquivo ainda.
                // INSERE o corpo da macro de volta na lista de leitura 'linhas'
                // para ser processado no próximo loop.
                let corpo = &deftab[indice].instrucoes;
                
                // Inserir ao contrário para manter a ordem certa na pilha
                for inst in corpo.iter().rev() {
                    linhas.insert(i, inst.clone());
                }
                
                // Se tinha um label antes (ex: INICIO DEF_SOM), escreve só o label
                if partes[0] != *palavra {
                    codigo_expandido.push_str(partes[0]);
                    codigo_expandido.push(' ');
                    // O restante da linha (a macro) será expandida nas próximas iterações
                }
                
                macro_encontrada = true;
                break;
            }
        }

        if !macro_encontrada {
            codigo_expandido.push_str(&linha);
            codigo_expandido.push('\n');
        }
    }

    let mut saida = File::create("MASMAPRG.ASM")?;
    saida.write_all(codigo_expandido.as_bytes())?;
    
    Ok(())
}