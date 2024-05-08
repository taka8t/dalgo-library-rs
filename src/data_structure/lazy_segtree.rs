pub struct LazySegtree<S, T, F, G, H> 
{
    n: usize,
    size: usize,
    log: usize,
    data: Vec<S>,
    lz: Vec<T>,
    op: F,
    e: S,
    mapping: G,
    composition: H,
    id: T,
}

impl<S, T, F, G, H> LazySegtree<S, T, F, G, H> 
where
    S: Copy,
    T: Copy,
    F: Fn(S, S) -> S,
    G: Fn(T, S) -> S,
    H: Fn(T, T) -> T
{
    pub fn new(n: usize, op: F, e: S, mapping: G, composition: H, id: T) -> Self {
        let size = n.next_power_of_two();
        let log = size.trailing_zeros() as usize;
        Self {
            n,
            size,
            log,
            data: vec![e; size * 2],
            op,
            e,
            lz: vec![id; size],
            mapping,
            composition,
            id,
        }
    }

    pub fn from(v: Vec<S>, op: F, e: S, mapping: G, composition: H, id: T) -> Self {
        let n = v.len();
        let size = n.next_power_of_two();
        let log = size.trailing_zeros() as usize;
        let mut data = vec![e; 2 * size];
        let lz = vec![id; size];
        data[size..(size + n)].clone_from_slice(&v);
        let mut ret = Self {
            n,
            size,
            log,
            data,
            op,
            e,
            lz,
            mapping,
            composition,
            id,
        };
        for i in (1..size).rev() {
            ret.update(i);
        }
        ret
    }

    pub fn set(&mut self, mut p: usize, x: S) {
        assert!(p < self.n);
        p += self.size;
        for i in (1..=self.log).rev() {
            self.push(p >> i);
        }
        self.data[p] = x;
        for i in 1..=self.log {
            self.update(p >> i);
        }
    }

    pub fn get(&mut self, mut p: usize) -> S {
        assert!(p < self.n);
        p += self.size;
        for i in (1..=self.log).rev() {
            self.push(p >> i);
        }
        self.data[p]
    }

    pub fn prod(&mut self, left: usize, right: usize) -> S {
        assert!(left <= right);
        assert!(right <= self.n);
        if left == right {
            return self.e;
        }
        let mut l = left + self.size;
        let mut r = right + self.size;
        for i in (1..=self.log).rev() {
            if ((l >> i) << i) != l {
                self.push(l >> i);
            }
            if ((r >> i) << i) != r {
                self.push(r >> i);
            }
        }
        let mut sml = self.e;
        let mut smr = self.e;
        while l < r {
            if l & 1 == 1 {
                sml = (self.op)(sml, self.data[l]);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                smr = (self.op)(self.data[r], smr);
            }
            l >>= 1;
            r >>= 1;
        }
        (self.op)(sml, smr)
    }

    pub fn all_prod(&self) -> S {
        self.data[1]
    }

    pub fn apply(&mut self, mut p: usize, f: T) {
        assert!(p < self.n);
        p += self.size;
        for i in (1..=self.log).rev() {
            self.push(p >> i);
        }
        self.data[p] = (self.mapping)(f, self.data[p]);
        for i in 1..=self.log {
            self.update(p >> i);
        }
    }

    pub fn apply_range(&mut self, left: usize, right: usize, f: T) {
        assert!(left <= right);
        assert!(right <= self.n);
        if left == right {
            return;
        }
        let mut l = left + self.size;
        let mut r = right + self.size;
        for i in (1..=self.log).rev() {
            if ((l >> i) << i) != l {
                self.push(l >> i);
            }
            if ((r >> i) << i) != r {
                self.push((r - 1) >> i);
            }
        }
        {
            let l2 = l;
            let r2 = r;
            while l < r {
                if l & 1 != 0 {
                    self.all_apply(l, f);
                    l += 1;
                }
                if r & 1 != 0 {
                    r -= 1;
                    self.all_apply(r, f);
                }
                l >>= 1;
                r >>= 1;
            }
            l = l2;
            r = r2;
        }
        for i in 1..=self.log {
            if ((l >> i) << i) != l {
                self.update(l >> i);
            }
            if ((r >> i) << i) != r {
                self.update((r - 1) >> i);
            }
        }
    }

    fn update(&mut self, k: usize) {
        self.data[k] = (self.op)(self.data[2 * k], self.data[2 * k + 1]);
    }

    fn all_apply(&mut self, k: usize, f: T) {
        self.data[k] = (self.mapping)(f, self.data[k]);
        if k < self.size {
            self.lz[k] = (self.composition)(f, self.lz[k]);
        }
    }

    fn push(&mut self, k: usize) {
        self.all_apply(2 * k, self.lz[k]);
        self.all_apply(2 * k + 1, self.lz[k]);
        self.lz[k] = self.id;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    const MOD: usize = 998244353;
    #[test]
    fn test_lazy_segtree(){
        let a = [1,2,3,4,5].into_iter().map(|x| (x, 1)).collect::<Vec<_>>();
        let mut st = LazySegtree::from(
            a,
            |x, y| ((x.0+y.0)%MOD, (x.1+y.1)%MOD),
            (0,0),
            |x, y| ((x.0*y.0+x.1*y.1)%MOD, y.1),
            |x, y| ((x.0*y.0)%MOD, (x.0*y.1+x.1)%MOD),
            (1,0)
        );
        let query = vec![
            (1, 0, 5, 0_, 0_),
            (0, 2, 4, 100, 101),
            (1, 0, 3, 0_, 0_),
            (0, 1, 3, 102, 103),
            (1, 2, 5, 0_, 0_),
            (0, 2, 5, 104, 105),
            (1, 0, 5, 0_, 0_)
        ];
        let ans = vec![
            15,
            0_,
            404,
            0_,
            41511,
            0_,
            4317767            
        ];
        for i in 0..7 {
            let t = query[i].0;
            if t == 0 {
                let (l, r, b, c) = (query[i].1, query[i].2, query[i].3, query[i].4);
                st.apply_range(l, r, (b,c));
            }
            else {
                let (l, r) = (query[i].1, query[i].2);
                assert_eq!(st.prod(l, r).0%MOD, ans[i]);
            }
        }
    }
}