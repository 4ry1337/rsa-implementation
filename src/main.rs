use crate::extended_precision_int::ExtendedPrecisionInt;
use crate::rsa::*;

mod extended_precision_int;
mod rsa;
mod utils;

fn main() {
    let rsa = RSA::new(3, 7);
    let m = ExtendedPrecisionInt::from("2");
    println!("{m}");
    let em = rsa.encryption(m);
    println!("{em}");
    let dm = rsa.decryption(em);
    println!("{dm}")
}
