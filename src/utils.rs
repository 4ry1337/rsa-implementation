pub fn is_prime(n: u32) -> bool {
    let limit = (n as f64).sqrt() as u32;
    for i in 2..=limit {
        if n % i == 0 {
            return false;
        }
    }
    return true;
}

pub fn gcd(a: u32, b: u32) -> u32 {
    if a == 0 {
        return b;
    }
    gcd(b % a, a)
}
