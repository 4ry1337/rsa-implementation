use crate::extended_precision_int::*;
use crate::utils::*;

pub struct RSA {
    pub n: ExtendedPrecisionInt,
    pub e: Vec<u32>,
    d: u32,
}

impl RSA {
    pub fn new(p: u32, q: u32) -> Self {
        if !is_prime(p) || !is_prime(q) {
            println!("p and q must be prime numbers");
        }
        let n = p * q;
        let mut e: Vec<u32> = vec![];
        let totient = (p - 1) * (q - 1);
        for i in 2..totient {
            if gcd(i, totient) == 1 {
                e.push(i);
            }
        }
        let x = e[0];
        let mut i = 0;
        let d = loop {
            if (i * x) % totient == 1 {
                break i;
            }
            i += 1;
        };

        Self {
            n: ExtendedPrecisionInt::from(&n.to_string()),
            e,
            d,
        }
    }

    pub fn encryption(&self, m: ExtendedPrecisionInt) -> ExtendedPrecisionInt {
        let mut em = ExtendedPrecisionInt::from("1");
        for _ in 0..self.e[0] {
            em = em * m.clone();
        }
        em = em % self.n.clone();
        em
    }

    pub fn decryption(&self, em: ExtendedPrecisionInt) -> ExtendedPrecisionInt {
        let mut dm = ExtendedPrecisionInt::from("1");
        for _ in 0..self.d {
            dm = dm * em.clone();
        }
        dm = dm % self.n.clone();
        dm
    }
}
