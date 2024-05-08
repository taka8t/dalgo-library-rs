use std::ops::{Add, Sub, AddAssign};

pub struct FenwickTree<T>
where
    T: AddAssign + Add<Output = T> + Sub<Output = T> + Copy,
{
    n: usize,
    data: Vec<T>,
    e: T
}

impl<T> FenwickTree<T>
where
    T: AddAssign + Add<Output = T> + Sub<Output = T> + Copy,
{
    pub fn new(n: usize, e: T) -> Self{
        Self {
            n: n,
            data: vec![e; n+1],
            e: e,
        }
    }

    pub fn add(&mut self, mut id: usize, x: T) {
        while id <= self.n {
            self.data[id] += x;
            id += id & !id + 1;
        }
    }

    // [0, r]
    pub fn sum(&self, mut r: usize) -> T {
        let mut res: T = self.e;
        while r != 0 {
            res += self.data[r];
            r &= r - 1;
        }
        res
    }

    // [l, r]
    pub fn sum2(&self, l: usize, r: usize) -> T {
        self.sum(r) - self.sum(l-1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fw_test() {
        let mut bt = FenwickTree::new(3, 0);
        bt.add(1, 1);
        bt.add(2, 2);
        bt.add(3, 3);
        assert_eq!(bt.sum(2), 3);
        assert_eq!(bt.sum2(1, 2), 3);
        assert_eq!(bt.sum(3), 6);
        assert_eq!(bt.sum2(2, 2), 2);
    }
}