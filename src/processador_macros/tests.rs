use crate::processador_macros::macros::processar;

#[test]
fn sem_parametros() {
    let sem_parametros = include_str!("../../programas_exemplo/macros/sem_parametros.asm");
    let saida = include_str!("../../programas_exemplo/macros/saida_sem_parametros.asm");
    assert_eq!(processar(sem_parametros).unwrap(), saida);
}

#[test]
fn com_parametros() {
    let com_parametros = include_str!("../../programas_exemplo/macros/macro.asm");
    let saida = include_str!("../../programas_exemplo/macros/saida_macro.asm");
    assert_eq!(processar(com_parametros).unwrap(), saida);
}

#[test]
fn multiplas_definicoes() {
    let multiplas_definicoes =
        include_str!("../../programas_exemplo/macros/multiplas_definicoes.asm");
    let saida = include_str!("../../programas_exemplo/macros/saida_multiplas_definicoes.asm");
    assert_eq!(processar(multiplas_definicoes).unwrap(), saida);
}
