use crate::montador::montador::primeiro_passo;
use std::collections::HashMap;

#[test]
fn primeiro_passo_add() {
    let add = include_str!("../../programas_exemplo/add.asm");
    let mut simbolos = HashMap::with_capacity(2);
    simbolos.insert("INICIO", 1000);
    simbolos.insert("STORE", 1006);

    assert_eq!(primeiro_passo(add), simbolos);
}
