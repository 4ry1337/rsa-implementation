use crate::extended_precision_int::ExtendedPrecisionInt;
use crate::rsa::*;

mod extended_precision_int;

mod rsa;

fn main() {
    let em = encryption(
        ExtendedPrecisionInt::from("21"),
        5,
        ExtendedPrecisionInt::from("18"),
    );
    println!("{em}");
    let dm = decryption(ExtendedPrecisionInt::from("21"), 5, em);
    println!("{dm}")
}
