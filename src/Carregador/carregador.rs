use crate::maquina::maquina::Maquina;
use crate::loader::estrutura::{modulo_carregavel, registro_modificações};
use anyhow::{Context, anyhow};
use std::collections::HashMap;
use std::mem::offset_of;

//=================================================================================================
pub struct Carregador {

    pub modulo_carregado: vec<modulo_carregavel>,
    tabela_simbolos: HashMap<String, u32>
}

//=================================================================================================



impl Carregador {
    pub fn new() -> self{
        Self{
            modulo_carregado: vec::new(),
            tabela_simbolos: HashMap::new(),
        }
    }

    //-----------------------------------------------------------------------------
    //carrega um unico arquivo objeto na memória no endereço especifico
    pub fn carregar_objeto(&mut self, codigo_obj: &str, base_endereço: u32, maquína: &mut Maquina)-> anyhow::Result<()>{


        //analisa o arquivo objeto
        let modulo = self.analisar_arquivo_obj(codigo_obj, base_endereço)?;

        //carrega o codigo na memória da maquina
        self.carregar_modulo_a_memoria(&modulo, maquína)?;

        //adiciona modulo para lista de modulos carregados
        self.modulo_carregado.push(modulo);

        Ok(());
    }
    //-----------------------------------------------------------------------------

    //analisa um codigo obj no formato do assembler
    fn analisar_arquivo_obj(&mut self, codigo_obj: &str, base_endereço: u32) -> anyhow::Result<ModuloCarregavel> {
        let linhas: Vec<&str> = codigo_obj.lines().collect();

        let mut nome = String::new();
        let mut endereço_inicial = base_endereço;
        let mut tamanho_programa = 0;
        let mut bytes_codigo = Vec::new();
        let mut modificações = Vec::new();
        let mut ponto_entrada = None;

        for line in linhas {
            if line.is_empty() {
                continue;
            }

            match line.chars().next() {
                Some('H') => self.analisar_registro_h(line, &mut nome, &mut endereço_inicial, &mut tamanho_programa)?,
                Some('D') => self.analisar_registro_d(line)?,
                Some('R') => self.analisar_registro_r(line)?,
                Some('T') => self.analisar_registro_t(line, &mut bytes_codigo)?,
                Some('M') => self.analisar_registro_m(line, &mut modificações)?,
                Some('E') => self.analisar_registro_e(line, &mut ponto_entrada)?,
                _ => {} // ignora outras linhas
            }
        }

        Ok(ModuloCarregavel {
            nome,
            endereço_inicial: base_endereço,
            codigo: bytes_codigo,
            registro_modificação: modificações,
            ponto_entrada,
        })
    }

//-----------------------------------------------------------------------------

//analisar registro h (header, nome , inicio, tamanho)

fn analisar_registro_h(&self, linha: &str, nome: &mut String, inicio: &mut u32, tamanho: &mut u32)->anyhow::Result<()>{

    if linha.len() <19 {
    return Err(anyhow!("Registro H invalido: {}", linha));
}

    *nome=linha[1..7].trim().to_string();

    //analisa endereço inicial (6 hexa)
    let string_inicial=&linha[7..13];
    *inicio=u32::from_str_radix(string_inic, 16)
        .context(format!("Endereço inial invalido em registro H: {}", string_inic))?;

    //analisa tamanho do programa (6 hexa)
    let string_tamanho=&linha[13..19];
    *tamanho=u32::from_str_radix(string_tamanho, 16)
        .context(format!("Tamanho de programa invalido no registro H: {}", string_tamanho))?;

    Ok(())
}

//-----------------------------------------------------------------------------

    // Analisa registro D (definições)
    fn analisar_registro_d(&mut self, linha: &str) -> anyhow::Result<()> {
        if linha.len() < 7 {
            return Err(anyhow!("Registro D invalido: {}", linha));
        }

        let mut i = 1; // Start after 'D'
        while i + 12 <= linha.len() {
            // Pegar endereço do simbolo ( 6 characteres)
            let nome_simbolo = linha[i..i+6].trim().to_string();

            // pegar endereço do simbolo (6 hexa)
            let endereco_str = &linha[i+6..i+12];
            let endereco = u32::from_str_radix(endereco_str, 16)
                .context(format!("Endereço invalido no registro D: {}", endereco_str))?;

            if !nome_simbolo.is_empty() {
                // checagem de simbolos dublicados
                if self.tabela_simbolos.contains_key(&nome_simbolo) {
                    return Err(anyhow!("Símbolo duplicado no registro D: {}", nome_simbolo));
                }

                // guardar na tabela
                self.tabela_simbolos.insert(nome_simbolo, endereco);
            }

            i += 12; // ir para próxima definição de simbolo
        }

        Ok(())
    }

    //-----------------------------------------------------------------------------
    // Analisa registro R (referências externas)
    fn analisar_registro_r(&mut self, linha: &str) -> anyhow::Result<()> {
        if linha.len() < 7 {
            return Err(anyhow!("Registro R invalido: {}", linha));
        }

        let mut i = 1; // começar depois de  'R'
        while i + 6 <= linha.len() {
            // Pegar endereço do simbolo ( 6 characteres)
            let nome_simbolo = linha[i..i+6].trim().to_string();

            if !nome_simbolo.is_empty() {
                // referencia externa é adionada com valor nulo/None
                // devem ser resolvidas com os modulos de ligador
                if !self.tabela_simbolos.contains_key(&nome_simbolo) {
                    // insere com placeholder (0) deve ser resolvidas durante o linking
                    self.tabela_simbolos.insert(nome_simbolo, 0);
                }
            }

            i += 6; // mover para proxima referencia de simbolo
        }

        Ok(())
    }


//-----------------------------------------------------------------------------

//analisa registro t (texto)
fn analisar_registro_t(&self, linha: &str, bytes_codigo: &mut Vec<u8>) -> anyhow::Result<()> {
    if linha.len() < 9 {
        return Err(anyhow!("Registro T invalido: {}", linha));
    }

    // analisa endereçoes (6 hexa)
    let _string_endereço = &linha[1..7];
    let _registro_endereço = u32::from_str_radix(_string_endereço, 16)
        .context(format!("Endereço invalido em registro T: {}", _string_endereço))?;

    // analisa tamanho (2 hexa)
    let string_tamanho = &linha[7..9];
    let registro_tamanho = usize::from_str_radix(string_tamanho, 16)
        .context(format!("Tamanho invalido em registro T: {}", string_tamanho))?;

    // analisa byte de dado
    if linha.len() < 9 + registro_tamanho * 2 {
        return Err(anyhow!("Informação/Data em registro T tem diferença de tamanho"));
    }

    let string_dado = &linha[9..];
    let mut i = 0;
    while i < string_dado.len() && bytes_codigo.len() < registro_tamanho {
        if i + 2 <= string_dado.len() {
            let string_byte = &string_dado[i..i + 2];
            let byte = u8::from_str_radix(string_byte, 16)
                .context(format!("Byte invalido no registro T {}", string_byte))?;
            bytes_codigo.push(byte);
            i += 2;
        } else {
            break;
        }
    }

    if bytes_codigo.len() < registro_tamanho {
        return Err(anyhow!("Informação/Data insuficiente no registro T"));
    }
    Ok(())
}

//-----------------------------------------------------------------------------

//analisa registro M (modificações)
fn analisar_registro_m(&self, linha: &str, modificações: &mut Vec<RegistroModificações>) -> anyhow::Result<()> {
    if linha.len() < 10 {
        return Err(anyhow!("Registro M invalido: {}", linha));
    }

    // analisa endereçoes (6 hexa)
    let string_endereço = &linha[1..7];
    let endereço = u32::from_str_radix(string_endereço, 16)
        .context(format!("Endereço invalido no registro M: {}", string_endereço))?;

    // analisa tamanho de modificações (2 hexa)
    let string_tamanho = &linha[7..9];
    let tamanho = u8::from_str_radix(string_tamanho, 16)
        .context(format!("Tamanho de modificação invalida no registro M: {}", string_tamanho))?;

    // Pegar operação (+ ou -)
    let operação = linha.chars().nth(9).unwrap_or('+');
    let simbolo = linha[10..].trim().to_string();

    modificações.push(RegistroModificações {
        endereço,
        tamanho,
        operação,
        simbolo,
    });

    Ok(())
}

//-----------------------------------------------------------------------------
//analisar registro E (End)
fn analisar_registro_e(&self, linha: &str, ponto_entrada: &mut Option<u32>) -> anyhow::Result<()> {
    if linha.len() > 1 {
        let string_endereço = &linha[1..7];
        let ponto_inicial = u32::from_str_radix(string_endereço, 16)
            .context(format!("Ponto inicial invalido em registro E: {}", string_endereço))?;
        *ponto_entrada = Some(ponto_inicial);
    }
    Ok(())
}

    //-----------------------------------------------------------------------------
    //carregar modulo do codigo na memória
    fn carregar_modulo_a_memoria(&self, modulo: &modulo_carregavel,maquina: &mut Maquina ) -> anyhow::Result<()>{

        let inicio=modulo.endereço_inicial as usize;
        let fim = inicio + modulo.codigo.len();

        if end > maquina.memoria().len() {
            return Err(anyhow!("Modulo grande demais para a memória"));
        }

        //pegar acesso mutavel a memória

        self.escrever_memoria(maquina, inicio, &modulo.codigo)

    }

    //suporte para escrever na memória de maquina

    fn escrever_memoria(&self, maquina: &mut Maquina, endereço: usize, dado: &[u8]) -> anyhow::Result<()>{
        //metodo para acessar memória
        if endereço == 0x6000 {
            maquina.carregar(dado)?;
            Ok(())
        }else{
            Err(anyhow!("Incapas de escrever no endereço no momento"))
        }
    }

    //-----------------------------------------------------------------------------
    //aplicar modificações para o código carregado
    pub fn aplicar_modificações(&mut self, maquina: &mut Maquina) ->anyhow::Result<()>{
        for modulo in &self.modulo_carregado{
            for modificação in &modulo.registro_modificações {
                self.aplicar_modificações(modulo, modificação, maquina)?;
            }
        }
        OK(())
    }

    //aplicar unico registro de modificação
    fn aplicar_modificação(&self, modulo: &ModuloCarregavel, reg_mod: &RegistroModificações, maquina: &mut Maquina) -> anyhow::Result<()> {
        // pegar endereço de simbolo da tabela
        let endereço_simbolo = self.tabela_simbolos.get(&reg_mod.simbolo)
            .copied()
            .context(format!("Símbolo não definido: {}", reg_mod.simbolo))?;

        let endereço_alvo = modulo.endereço_inicial + reg_mod.endereço;

        // ler valor atual da memória
        let mut valor_atual = 0u64;
        for i in 0..reg_mod.tamanho as usize {
            let byte = self.ler_byte_memoria(maquina, endereço_alvo as usize + i)?;
            valor_atual = (valor_atual << 8) | (byte as u64);
        }

        // aplicar modificações
        let novo_valor = match reg_mod.operação {
            '+' => valor_atual + endereço_simbolo as u64,
            '-' => valor_atual - endereço_simbolo as u64,
            _ => return Err(anyhow!("Operação de modificação invalida")),
        };

        // escrever de volta
        self.escrever_valor_memoria(maquina, endereço_alvo as usize, novo_valor, reg_mod.tamanho as usize)?;

        Ok(())
    }

    //-----------------------------------------------------------------------------

    //ler um byte da memória
    fn ler_byte_memoria(&self, maquina: &Maquina, endereço: usize) -> anyhow::Result<(u8)>{
        maquina.memoria().get(endereço).copied()
            .context(format!("Leitura de memória fora dos limites em 0x{:04X}", endereço))
    }

    //escrever multi-byte a memória
    fn escrever_valor_memoria(&self, maquina: &mut Maquina, endereço: usize, valor: u64, tamanho: usize) -> anyhow::Result<()>{
        #[cfg(debug_assertions)]
        {
            eprintln!("Aviso: usando memória não segura em  0x{:04X}", endereço);
        }
        unsafe {
            let ponteiro_memoria = (maquina as &mut Maquina).cast::<u8>();
            let memoria_offset = ponteiro_memori.add(std::mem:offset_of!(Maquina, memoria));

            for i in 0..tamanho{
                let byte = ((valor >> (8* (tamanho -1 -i))) & 0xFF) as u8;
                std::ptr::write(memoria_offset.add(endereço+i),byte);
            }
        }

        Ok(())
    }
    //resetar o carregador
    pub fn reset(&mut self){
        self.modulo_carregados.clear();
        self.tabela_simbolos.clear();
    }
}


