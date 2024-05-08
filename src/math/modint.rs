#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ModInt<const MOD: usize> {
    val: usize,
}

impl<const MOD: usize> ModInt<MOD> {
    pub fn new(n: usize) -> Self {
        Self {val: n % MOD}
    }

    pub fn val(&self) -> usize {
        self.val
    }

    pub fn pow(&self, mut n: usize) -> Self {
        let mut ret = 1;
        let mut x = self.val;
        while n > 0 {
            if (n & 1) == 1 {ret = ret * x % MOD;}
            x = x * x % MOD;
            n >>= 1;
        }
        Self {val: ret}
    }

    pub fn inv(&self) -> Self {
        self.pow(MOD - 2)
    }
}

impl<const MOD: usize> From<usize> for ModInt<MOD> {
    fn from(item: usize) -> Self {
        Self {val: item % MOD}
    }
}

impl<const MOD: usize> std::fmt::Display for ModInt<MOD> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.val)
    }
}

impl<const MOD: usize> std::ops::Add for ModInt<MOD> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let mut s = self.val + rhs.val;
        if s > MOD {s -= MOD;}
        Self {val: s}
    }
}

impl<const MOD: usize> std::ops::AddAssign for ModInt<MOD> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<const MOD: usize> std::ops::Sub for ModInt<MOD> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {val: if self.val > rhs.val {self.val - rhs.val} else {MOD + self.val - rhs.val}}
    }
}

impl<const MOD: usize> std::ops::SubAssign for ModInt<MOD> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl<const MOD: usize> std::ops::Mul for ModInt<MOD> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self {val: self.val * rhs.val % MOD}
    }
}

impl<const MOD: usize> std::ops::MulAssign for ModInt<MOD> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

pub struct Factrial<const MOD: usize> {
    fact: Vec<ModInt<MOD>>,
    finv: Vec<ModInt<MOD>>,
}

impl<const MOD: usize> Factrial<MOD> {
    pub fn new(n: usize) -> Self {
        let mut fact = vec![0.into(); n+1];
        let mut inv = vec![0.into(); n+1];
        let mut finv = vec![0.into(); n+1];
        inv[1] = 1.into();
        for i in 2..n+1 {inv[i] = Into::<ModInt<MOD>>::into(MOD - (MOD / i)) * inv[MOD % i];}
        fact[0] = 1.into();
        finv[0] = 1.into();
        for i in 1..n+1 {
            fact[i] = fact[i-1] * i.into();
            finv[i] = finv[i-1] * inv[i];
        }
        Self {
            fact, finv,
        }
    }

    pub fn comb (&self, n: usize, k: usize) -> ModInt<MOD> {
        if n < k {return 0.into();}
        self.finv[k] * self.finv[n-k] * self.fact[n]
    }

    pub fn perm (&self, n: usize, k: usize) -> ModInt<MOD> {
        if n < k {return 0.into();}
        self.finv[n-k] * self.fact[n]
    }
}


#[inline]
pub fn modpow (mut x: usize, mut n: usize, m: usize) -> usize {
    let mut ret = 1;
    while n > 0 {
        if (n & 1) == 1 {ret = ret * x % m;}
        x = x * x % m;
        n >>= 1;
    }
    ret
}

#[inline]
pub fn modpowll(mut x: usize, mut n: usize, m: usize) -> usize {
    let mut ret = 1;
    while n > 0 {
        if (n & 1) == 1 {ret = ((ret as u128) * (x as u128) % (m as u128)) as usize;}
        x = ((x as u128) * (x as u128) % (m as u128)) as usize;
        n >>= 1;
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    const MOD1: usize = 1000000007;
    const MOD2: usize = 998244353;
    const MOD3: usize = 7;

    #[test]
    fn test_modpow() {
        assert_eq!(modpow(8, 24, MOD1), 80065005);
        assert_eq!(modpow(8, 24, MOD2), 24888593);
        assert_eq!(modpow(8, 24, MOD3), 1);

        assert_eq!(modpow(0, 12, MOD1), 0);
        assert_eq!(modpow(0, 12, MOD2), 0);
        assert_eq!(modpow(0, 12, MOD3), 0);

        assert_eq!(modpow(0, 1, MOD1), 0);
        assert_eq!(modpow(0, 0, MOD1), 1);
        assert_eq!(modpow(5, 0, MOD2), 1);
        assert_eq!(modpow(MOD3, 1, MOD3), 0);

        assert_eq!(modpow(314, 1592, MOD1), 821477295);
        assert_eq!(modpow(314, 1592, MOD2), 648812915);
        assert_eq!(modpow(314, 1592, MOD3), 1);
    }

    #[test]
    fn modint_ops() {
        let a = ModInt::<MOD1>::new(8);
        let b = ModInt::<MOD1>::new(24);
        assert_eq!((a+b).val(), 8+24);
        assert_eq!((a-b).val(), MOD1+8-24);
        assert_eq!((b-a).val(), 24-8);
        assert_eq!((a*b).val(), 24*8);

        let a = ModInt::<MOD2>::new(8);
        let b = ModInt::<MOD2>::new(24);
        assert_eq!((a+b).val(), 8+24);
        assert_eq!((a-b).val(), MOD2+8-24);
        assert_eq!((b-a).val(), 24-8);
        assert_eq!((a*b).val(), 24*8);

        let a = ModInt::<MOD3>::new(8);
        let b = ModInt::<MOD3>::new(24);
        assert_eq!((a+b).val(), (8+24)%MOD3);
        assert_eq!((a-b).val(), MOD3+8%MOD3-24%MOD3);
        assert_eq!((b-a).val(), (24-8)%MOD3);
        assert_eq!((a*b).val(), 24*8%MOD3);
    }

    #[test]
    fn modint_pow() {
        let a = ModInt::<MOD1>::new(8);
        assert_eq!(a.pow(24).val(), modpow(8, 24, MOD1));

        let a = ModInt::<MOD2>::new(8);
        assert_eq!(a.pow(24).val(), modpow(8, 24, MOD2));

        let a = ModInt::<MOD3>::new(8);
        assert_eq!(a.pow(24).val(), modpow(8, 24, MOD3));
    }

    #[test]
    fn modint_inv() {
        let a = ModInt::<MOD1>::new(8);
        assert_eq!(a.inv().val(), modpow(8, MOD1-2, MOD1));

        let a = ModInt::<MOD2>::new(8);
        assert_eq!(a.inv().val(), modpow(8, MOD2-2, MOD2));

        let a = ModInt::<MOD3>::new(8);
        assert_eq!(a.inv().val(), modpow(8, MOD3-2, MOD3));
    }

    #[test]
    fn test_perm() {
        let ft = Factrial::<MOD1>::new(10000);
        assert_eq!(ft.perm(8, 3).val(), 336);
        assert_eq!(ft.perm(314, 159).val(), 720296313);
        assert_eq!(ft.perm(9, 0).val(), 1);
    }

    #[test]
    fn test_comb() {
        let ft = Factrial::<MOD1>::new(10000);
        assert_eq!(ft.comb(8, 3).val(), 56);
        assert_eq!(ft.comb(314, 159).val(), 538085512);
        assert_eq!(ft.comb(9, 0).val(), 1);
    }
}