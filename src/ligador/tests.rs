use crate::ligador::ligador::ligar_objeto;
use crate::montador::montador::montar;

#[test]
fn ligar() {
    let secoes = include_str!("../../programas_teste/secoes.asm");
    let saida = montar(secoes).unwrap();
    assert_eq!(ligar_objeto(&saida).unwrap(), "1B100700190001B400")
}
