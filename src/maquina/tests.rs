use crate::maquina::constantes::registradores;
use crate::maquina::maquina::Maquina;

#[test]
fn add_imediato() {
    let mut maquina = Maquina::new();
    maquina.carregar(&[0x19, 0x00, 0x01]).unwrap();
    maquina.executar_instrucao().unwrap();
    assert_eq!(maquina.registrador(registradores::A), Some(1));
}

#[test]
fn clear() {
    let mut maquina = Maquina::new();
    maquina.carregar(&[0x19, 0x00, 0x01]).unwrap();
    maquina.executar_instrucao().unwrap();
    assert_eq!(maquina.registrador(registradores::A), Some(1));

    maquina.carregar(&[0x04, 0x00, 0x00]).unwrap();
    maquina.executar_instrucao().unwrap();
    assert_eq!(maquina.registrador(registradores::A), Some(0));
}

#[test]
fn programa_com_multiplas_instrucoes() {
    let mut maquina = Maquina::new();
    maquina
        .carregar(&[0x19, 0x00, 0x01, 0x19, 0x00, 0x01, 0x19, 0x00, 0x01])
        .unwrap();

    maquina.executar_instrucao().unwrap();
    assert_eq!(maquina.registrador(registradores::A), Some(1));

    maquina.executar_instrucao().unwrap();
    assert_eq!(maquina.registrador(registradores::A), Some(2));

    maquina.executar_instrucao().unwrap();
    assert_eq!(maquina.registrador(registradores::A), Some(3));

    // Assegurar que todas as instruções foram executadas
    assert!(maquina.executar_instrucao().is_err());
}
