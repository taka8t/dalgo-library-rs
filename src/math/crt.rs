// https://github.com/rust-lang-ja/ac-library-rs/blob/master/src/math.rs
pub fn crt(r: &[i64], m: &[i64]) -> (i64, i64) {
    let (mut r0, mut m0) = (0, 1);
    for (&(mut ri), &(mut mi)) in r.iter().zip(m.iter()) {
        ri = ri % mi;
        if ri < 0 {ri += mi;}
        if m0 < mi {
            std::mem::swap(&mut r0, &mut ri);
            std::mem::swap(&mut m0, &mut mi);
        }
        if m0 % mi == 0 {
            if r0 % mi != ri {
                return (0, 0);
            }
            continue;
        }

        let (g, im) = mod_inv(m0, mi);
        let u1 = mi / g;
        if (ri - r0) % g != 0 {
            return (0, 0);
        }
        let x = (ri - r0) / g % u1 * im % u1;

        r0 += x * m0;
        m0 *= u1;
        if r0 < 0 {
            r0 += m0
        };
    }

    (r0, m0)
}


#[inline]
pub fn exgcd(a: i64, b: i64, x: &mut i64, y: &mut i64) -> i64 {
    let mut d = a;
    if b != 0 {
        d = exgcd(b, a%b, y, x);
        *y -= a / b * *x;
    }
    else {
        *x = 1;
        *y = 0;
    }
    d
}

// ax ≡ g (mod m)
// (g, x)
#[inline]
pub fn mod_inv(a: i64, m: i64) -> (i64, i64) {
    let mut x = 1;
    let mut y = 0;
    (exgcd(a, m, &mut x, &mut y), (m + x % m) % m)
}

// \sum_{i = 0}^{N - 1} floor((A \times i + B) / M)
pub fn floor_sum(n: i64, m: i64, mut a: i64, mut b: i64) -> i64 {
    let mut ans = 0;
    if a >= m {
        ans += (n - 1) * n * (a / m) / 2;
        a %= m;
    }
    if b >= m {
        ans += n * (b / m);
        b %= m;
    }

    let y_max = (a * n + b) / m;
    let x_max = y_max * m - b;
    if y_max == 0 {
        return ans;
    }
    ans += (n - (x_max + a - 1) / a) * y_max;
    ans += floor_sum(y_max, a, m, (a - x_max % a) % a);
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crt(){
        let a = [44, 23, 13];
        let b = [13, 50, 22];
        assert_eq!(crt(&a, &b), (1773, 7150));
        let a = [12345, 67890, 99999];
        let b = [13, 444321, 95318];
        assert_eq!(crt(&a, &b), (103333581255, 550573258014));
        let a = [0, 3, 4];
        let b = [1, 9, 5];
        assert_eq!(crt(&a, &b), (39, 45));
    }

    #[test]
    fn test_floor_sum() {
        assert_eq!(floor_sum(0, 1, 0, 0), 0);
        assert_eq!(floor_sum(1_000_000_000, 1, 1, 1), 500_000_000_500_000_000);
        assert_eq!(
            floor_sum(1_000_000_000, 1_000_000_000, 999_999_999, 999_999_999),
            499_999_999_500_000_000
        );
        assert_eq!(floor_sum(332955, 5590132, 2231, 999423), 22014575);
    }
}