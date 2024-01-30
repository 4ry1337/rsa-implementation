use crate::extended_precision_int::*;

pub fn encryption<'a>(
    n: ExtendedPrecisionInt,
    e: u32,
    m: ExtendedPrecisionInt,
) -> ExtendedPrecisionInt<'a> {
    let mut em = ExtendedPrecisionInt::from("1");
    for _ in 0..e {
        em = em * m;
    }
    em = em % n;
    em
}

pub fn decryption<'a>(
    n: ExtendedPrecisionInt,
    d: u32,
    em: ExtendedPrecisionInt,
) -> ExtendedPrecisionInt<'a> {
    let mut dm = ExtendedPrecisionInt::from("1");
    for _ in 0..d {
        dm = dm * em;
    }
    dm = dm % n;
    dm
}
