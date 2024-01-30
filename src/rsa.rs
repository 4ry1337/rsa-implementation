use crate::extended_precision_int::*;

pub fn encryption<'a>(
    n: ExtendedPrecisionInt,
    e: u32,
    m: ExtendedPrecisionInt,
) -> ExtendedPrecisionInt {
    let mut em = ExtendedPrecisionInt::from("1");
    for _ in 0..e {
        em = em * m.clone();
    }
    em = em % n;
    em
}

pub fn decryption<'a>(
    n: ExtendedPrecisionInt,
    d: u32,
    em: ExtendedPrecisionInt,
) -> ExtendedPrecisionInt {
    let mut dm = ExtendedPrecisionInt::from("1");
    for _ in 0..d {
        dm = dm * em.clone();
    }
    dm = dm % n;
    dm
}
