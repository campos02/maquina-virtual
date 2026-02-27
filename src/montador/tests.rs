use crate::montador::montador::{Simbolo, TipoSimbolo, primeiro_passo, segundo_passo};
use std::collections::HashMap;

#[test]
fn primeiro_passo_add() {
    let add = include_str!("../../programas_exemplo/add.asm");
    let mut simbolos = HashMap::with_capacity(2);
    simbolos.insert(
        "INICIO",
        Simbolo {
            localizacao: Some(0x1000),
            tipo: TipoSimbolo::Local,
        },
    );

    simbolos.insert(
        "STORE",
        Simbolo {
            localizacao: Some(0x1006),
            tipo: TipoSimbolo::Local,
        },
    );

    let resultado = primeiro_passo(add).unwrap();
    assert_eq!(resultado.len(), 2);

    for (nome, simbolo) in resultado.iter() {
        assert_eq!(
            simbolos.get(nome).unwrap().localizacao.unwrap(),
            simbolo.localizacao.unwrap()
        );
    }
}

#[test]
fn primeiro_passo_byte() {
    let byte = include_str!("../../programas_exemplo/byte.asm");
    let mut simbolos = HashMap::with_capacity(2);
    simbolos.insert(
        "INICIO",
        Simbolo {
            localizacao: Some(0x1000),
            tipo: TipoSimbolo::Local,
        },
    );

    simbolos.insert(
        "ADD_1",
        Simbolo {
            localizacao: Some(0x1007),
            tipo: TipoSimbolo::Local,
        },
    );

    let resultado = primeiro_passo(byte).unwrap();
    assert_eq!(resultado.len(), 2);

    for (nome, simbolo) in resultado.iter() {
        assert_eq!(
            simbolos.get(nome).unwrap().localizacao.unwrap(),
            simbolo.localizacao.unwrap()
        );
    }
}

#[test]
fn montar_add() {
    let add = include_str!("../../programas_exemplo/add.asm");
    let simbolos = primeiro_passo(add).unwrap();

    assert_eq!(
        segundo_passo(add, &simbolos).unwrap(),
        "HT_ADD 00100000000B\n\
        T0010000B1900011900010D0000B400\n\
        E001000"
    );
}

#[test]
fn montar_externos() {
    let externos = include_str!("../../programas_exemplo/externos.asm");
    let simbolos = primeiro_passo(externos).unwrap();

    assert_eq!(
        segundo_passo(externos, &simbolos).unwrap(),
        "HT_ADD 00000000000E\n\
        RZERO  UM    \n\
        T0000000E1B1000001B1000000F100000B400\n\
        M00000102+UM\n\
        M00000502+UM\n\
        M00000904+ZERO\n\
        E000000"
    );
}
