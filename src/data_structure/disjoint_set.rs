pub struct DisjointSet {
    _n: usize,
    data: Vec<usize>,
    size: Vec<usize>
}

impl DisjointSet {
    pub fn new(n: usize) -> Self{
        Self {
            _n: n,
            data: (0..n).map(|i| i).collect::<Vec<usize>>(),
            size: vec![1; n],
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.data[x] == x {
            x
        }
        else {
            let px = self.data[x];
            self.data[x] = self.find(px);
            self.data[x]
        }
    }

    pub fn unite(&mut self, x: usize, y: usize) {
        let mut px = self.find(x);
        let mut py = self.find(y);
        if px == py {
            return;
        }
        if self.size[py] < self.size[px] {
            std::mem::swap(&mut px, &mut py);
        }
        self.data[px] = py;
        self.size[py] += self.size[px];
        self._n -= 1;
    }

    pub fn same(&mut self, x: usize, y: usize) -> bool {
        let px = self.find(x);
        let py = self.find(y);
        self.data[px] == self.data[py]
    }

    pub fn num_trees(&self) -> usize {
        self._n
    }

    pub fn size(&mut self, x: usize) -> usize {
        let px = self.find(self.data[x]);
        self.size[px]
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dsu_test() {
        let mut uf = DisjointSet::new(4);
        uf.unite(0, 1);
        assert!(uf.same(0, 1));
        uf.unite(1, 2);
        assert!(uf.same(0, 2));
        assert_eq!(uf.size(0), 3);
        assert!(!uf.same(0, 3));
        assert_eq!(uf.num_trees(), 2);
        assert_eq!(uf.size(1), 3);
        assert_eq!(uf.size(3), 1);
    }

    #[test]
    fn kruscal_test() {
        let mut edges = 
        vec! [
            (2,1,0),
            (1,2,1),
            (1,2,3),
            (1,3,0),
            (3,0,2),
            (5,1,3),
        ];
        edges.sort();
        let mut uf = DisjointSet::new(4);
        let mut ans = 0;
        for (cost, v1, v2) in edges {
            if !uf.same(v1, v2) {
                ans += cost;
                uf.unite(v1, v2);
            }
        }
        assert_eq!(ans, 3);


        let mut edges = 
        vec! [
            (1,1,0),
            (3,2,0),
            (1,2,1),
            (7,3,1),
            (1,4,2),
            (3,1,4),
            (1,3,4),
            (1,3,5),
            (6,4,5),
        ];
        edges.sort();
        let mut uf = DisjointSet::new(6);
        let mut ans = 0;
        for (cost, v1, v2) in edges {
            if !uf.same(v1, v2) {
                ans += cost;
                uf.unite(v1, v2);
            }
        }
        assert_eq!(ans, 5);
    }
}