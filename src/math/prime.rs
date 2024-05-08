use crate::math::modint::{modpow, modpowll};

// n < 2^32
pub fn is_prime(n: usize) -> bool {
    match n {
        _ if n <= 1 => return false,
        2 | 7 | 61 => return true,
        _ if n % 2 == 0 => return false,
        _ => {}
    }
    let mut d = n - 1;
    while d % 2 == 0 {
        d /= 2;
    }
    for &a in &[2, 7, 61] {
        let mut t = d;
        let mut y = modpow(a, t, n);
        while t != n - 1 && y != 1 && y != n - 1 {
            y = y * y % n;
            t <<= 1;
        }
        if y != n - 1 && t % 2 == 0 {
            return false;
        }
    }
    true
}

pub fn is_primell(n: usize) -> bool {
    match n {
        _ if n <= 1 => return false,
        2 => return true,
        _ if n % 2 == 0 => return false,
        _ => {}
    }
    let mut d = n - 1;
    while d % 2 == 0 {
        d /= 2;
    }
    for &base in &[2, 325, 9375, 28178, 450775, 9780504, 1795265022] {
        let a = if base > n {base % n} else {base};
        if a == 0 {return true;}
        let mut t = d;
        let mut y = modpowll(a, t, n);
        while t != n - 1 && y != 1 && y != n - 1 {
            y = ((y as u128) * (y as u128) % (n as u128)) as usize;
            t <<= 1;
        }
        if y != n - 1 && t % 2 == 0 {
            return false;
        }
    }
    true
}

// primes, lpf
pub fn linear_sieve(n: usize) -> (Vec<usize>, Vec<usize>) {
    let mut lpf = vec![n+10;n+1];
    let nf = n as f64;
    let mut primes = Vec::with_capacity((nf / nf.ln() * 1.1) as usize);
    for i in 2..=n {
        if lpf[i] > n {
            primes.push(i);
            lpf[i] = i;
        }
        for &p in &primes {
            if i*p > n || lpf[i] < p {break;}
            lpf[i*p] = p;
        }
    }
    (primes, lpf)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_isprime() {
        assert!(!is_prime(1));
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(!is_prime(4));
        assert!(is_prime(5));
        assert!(is_prime(1000000007));
        assert!(is_prime(998244353));
        assert!(!is_prime(57));
        assert!(!is_prime(37765679));
        assert!(is_prime(104147));
        assert!(is_prime(104233));
        assert!(!is_prime(104719));

        assert!(!is_primell(104719));
    }

    #[test]
    fn test_linear_sieve() {
        let (prime, lpf) = linear_sieve(25);
        assert_eq!(prime, &[2,3,5,7,11,13,17,19,23]);
        assert_eq!(lpf[2..], [2,3,2,5,2,7,2,3,2,11,2,13,2,3,2,17,2,19,2,3,2,23,2,5]);
    }
}