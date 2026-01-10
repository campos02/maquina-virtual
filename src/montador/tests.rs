use crate::montador::montador::{primeiro_passo, segundo_passo};
use std::collections::HashMap;

#[test]
fn primeiro_passo_add() {
    let add = include_str!("../../programas_exemplo/add.asm");
    let mut simbolos = HashMap::with_capacity(2);
    simbolos.insert("INICIO", 0x1000);
    simbolos.insert("STORE", 0x1006);

    assert_eq!(primeiro_passo(add).unwrap(), simbolos);
}

#[test]
fn primeiro_passo_byte() {
    let byte = include_str!("../../programas_exemplo/byte.asm");
    let mut simbolos = HashMap::with_capacity(2);
    simbolos.insert("INICIO", 0x1000);
    simbolos.insert("ADD_1", 0x1007);

    assert_eq!(primeiro_passo(byte).unwrap(), simbolos);
}

#[test]
fn montar_add() {
    let add = include_str!("../../programas_exemplo/add.asm");
    let mut simbolos = HashMap::with_capacity(2);
    simbolos.insert("INICIO", 0x1000);
    simbolos.insert("STORE", 0x1006);

    assert_eq!(primeiro_passo(add).unwrap(), simbolos);
    assert_eq!(
        segundo_passo(add, &simbolos).unwrap(),
        "HT_ADD 00100000000B\nT0010000B1900011900010D0000B400\nE001000"
    );
}
