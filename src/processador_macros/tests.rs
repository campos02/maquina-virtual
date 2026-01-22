use crate::processador_macros::macros::processar;

#[test]
fn sem_argumentos() {
    let sem_argumentos = include_str!("../../programas_exemplo/macros/sem_argumentos.asm");
    let saida = include_str!("../../programas_exemplo/macros/saida_sem_argumentos.asm");
    assert_eq!(processar(sem_argumentos).unwrap(), saida);
}
