use crate::montador::montador::montar;

#[test]
fn montar_add() {
    let add = include_str!("../../programas_teste/add.asm");
    assert_eq!(
        montar(add).unwrap(),
        "HT_ADD 00100000000B\n\
        T0010000B1900011900010D0000B400\n\
        E001000\n\n"
    );
}

#[test]
fn montar_externos() {
    let externos = include_str!("../../programas_teste/externos.asm");
    let saida = montar(externos).unwrap();

    assert!(
        saida
            == "HT_ADD 00000000000E\n\
        RZERO  UM    \n\
        T0000000E1B1000001B1000000F100000B400\n\
        M00000102+UM\n\
        M00000502+UM\n\
        M00000904+ZERO\n\
        E000000\n\n"
            || saida
                == "HT_ADD 00000000000E\n\
        RUM    ZERO  \n\
        T0000000E1B1000001B1000000F100000B400\n\
        M00000102+UM\n\
        M00000502+UM\n\
        M00000904+ZERO\n\
        E000000\n\n"
    );
}

#[test]
fn montar_secoes() {
    let secoes = include_str!("../../programas_teste/secoes.asm");
    let saida = montar(secoes).unwrap();

    assert_eq!(
        saida,
        "HT_ADD 000000000007\n\
        RTESTE \n\
        T000000071B100000190001\n\
        M00000105+TESTE\n\
        E000000\n\n\
        HTESTE 000000000002\n\
        T00000002B400\n\
        E000000\n\n"
    );
}
