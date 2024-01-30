use crate::extended_precision_int::ExtendedPrecisionInt;
use crate::rsa::*;

mod extended_precision_int;

mod rsa;

fn main() {
    let n = ExtendedPrecisionInt::from("3713");
    let m = ExtendedPrecisionInt::from("2003");
    let e = 7;
    let d = 2563;
    let em = encryption(n, e, m);
    println!("{em}");
    let dm = decryption(n, d, em);
    println!("{dm}")
}
