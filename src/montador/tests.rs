use crate::montador::montador::primeiro_passo;
use std::collections::HashMap;

#[test]
fn primeiro_passo_add() {
    let add = include_str!("../../programas_exemplo/add.asm");
    let mut simbolos = HashMap::with_capacity(2);
    simbolos.insert("INICIO", 1000);
    simbolos.insert("STORE", 1006);

    assert_eq!(primeiro_passo(add).unwrap(), simbolos);
}

#[test]
fn primeiro_passo_byte() {
    let byte = include_str!("../../programas_exemplo/byte.asm");
    let mut simbolos = HashMap::with_capacity(2);
    simbolos.insert("INICIO", 1000);
    simbolos.insert("ADD_1", 1007);

    assert_eq!(primeiro_passo(byte).unwrap(), simbolos);
}
