use rand::Rng;
use std::{
    cmp::Ordering,
    fmt,
    ops::{Add, Div, Mul, Rem, Sub},
};

pub struct ExtendedPrecisionInt(Vec<u8>);

impl ExtendedPrecisionInt {
    pub fn from(digit_str: &str) -> ExtendedPrecisionInt {
        let mut digits: Vec<u8> = Vec::new();
        for c in digit_str.chars() {
            if let Some(digit) = c.to_digit(10) {
                digits.push(digit as u8)
            }
        }
        while digits.first() == Some(&0) {
            digits.remove(0);
        }
        Self(digits)
    }

    pub fn rand(n: u32) -> ExtendedPrecisionInt {
        let mut rng = rand::thread_rng();
        let max = rng.gen_range(1..n);
        let mut digits = Vec::new();
        digits.push(rng.gen_range(0..9));
        for _ in 1..max {
            digits.push(rng.gen_range(0..9));
        }
        Self(digits)
    }

    pub fn parity(&self) {
        if self.0[self.0.len() - 1] % 2 == 1 {
            println!("{} is odd", self)
        } else {
            println!("{} is even", self)
        }
    }
}

impl fmt::Display for ExtendedPrecisionInt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for digit in &self.0 {
            write!(f, "{}", digit)?;
        }
        Ok(())
    }
}

impl Add for ExtendedPrecisionInt {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let mut result = Vec::new();
        let mut carry = 0;
        let a: Vec<u8> = self.0.into_iter().rev().collect();
        let b: Vec<u8> = other.0.into_iter().rev().collect();
        let n = std::cmp::max(a.len(), b.len());
        for i in 0..n {
            let a_digit = if i < a.len() { a[i] } else { 0 };
            let b_digit = if i < b.len() { b[i] } else { 0 };
            let sum = a_digit + b_digit + carry;
            result.push(sum % 10);
            carry = sum / 10;
        }
        if carry > 0 {
            result.push(carry);
        }
        result.reverse();
        Self(result)
    }
}

impl Sub for ExtendedPrecisionInt {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        let mut result: Vec<u8> = Vec::new();
        let mut carry: u8 = 0;
        assert!(self >= rhs, "lhs must be larger than rhs");
        let l: Vec<u8> = self.0.into_iter().rev().collect();
        let r: Vec<u8> = rhs.0.into_iter().rev().collect();
        for i in 0..l.len() {
            let l_digit = l[i];
            let r_digit = if i < r.len() { r[i] + carry } else { 0 + carry };
            let sub: u8 = match l_digit.cmp(&r_digit) {
                Ordering::Less => {
                    carry = 1;
                    10 - (r_digit - l_digit)
                }
                _ => {
                    carry = 0;
                    l_digit - r_digit
                }
            };
            result.push(sub);
        }
        while result.last() == Some(&0) {
            result.pop();
        }
        result.reverse();
        Self(result)
    }
}

impl Mul for ExtendedPrecisionInt {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let mut result = vec![0; self.0.len() + rhs.0.len()];

        let l: Vec<u8> = self.0.into_iter().rev().collect();
        let r: Vec<u8> = rhs.0.into_iter().rev().collect();
        for (i, &a) in l.iter().enumerate() {
            let mut carry = 0;
            for (j, &b) in r.iter().enumerate() {
                let temp = a * b + result[i + j] + carry;
                result[i + j] = temp % 10;
                carry = temp / 10;
            }
            result[i + r.len()] += carry;
        }

        while result.last() == Some(&0) {
            result.pop();
        }

        result.reverse();

        Self(result)
    }
}

impl Div for ExtendedPrecisionInt {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        Self(div_helper(self, rhs).0 .0)
    }
}

impl Rem for ExtendedPrecisionInt {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self {
        Self(div_helper(self, rhs).1 .0)
    }
}

fn div_helper(
    a: ExtendedPrecisionInt,
    b: ExtendedPrecisionInt,
) -> (ExtendedPrecisionInt, ExtendedPrecisionInt) {
    if a < b {
        return (ExtendedPrecisionInt::from("0"), a);
    }
    let (mut q, r) = div_helper(a, ExtendedPrecisionInt::from("2") * b.clone());
    q = ExtendedPrecisionInt::from("2") * q;
    if r < b {
        return (q, r);
    } else {
        return (q + ExtendedPrecisionInt::from("1"), r - b);
    }
}

impl Ord for ExtendedPrecisionInt {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.len().cmp(&other.0.len()).then_with(|| {
            for (a, b) in self.0.iter().zip(other.0.iter()) {
                match a.cmp(b) {
                    Ordering::Equal => continue,
                    other => return other,
                }
            }
            Ordering::Equal
        })
    }
}

impl PartialOrd for ExtendedPrecisionInt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for ExtendedPrecisionInt {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for ExtendedPrecisionInt {}

impl Clone for ExtendedPrecisionInt {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
