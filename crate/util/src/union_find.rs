pub struct UnionFind {
    pub parent: Vec<usize>,
    pub rank: Vec<usize>,
    pub components: usize,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        let mut rank = Vec::new();
        rank.resize(size, 1);
        Self {
            parent: (0..size).collect(),
            rank,
            components: size,
        }
    }

    pub fn find(&mut self, a: usize) -> usize {
        let ap = self.parent[a];
        if a == ap {
            return a;
        }

        let app = self.find(ap);
        self.parent[a] = app;
        app
    }

    pub fn union(&mut self, a: usize, b: usize) {
        let (ap, bp) = (self.find(a), self.find(b));
        if ap == bp {
            return;
        }

        let (apr, bpr) = (self.rank[ap], self.rank[bp]);
        if apr > bpr {
            self.parent[bp] = ap;
            self.rank[ap] += bpr;
        } else {
            self.parent[ap] = bp;
            self.rank[bp] += apr;
        }
        self.components -= 1;
    }

    pub fn is_connected(&self) -> bool {
        self.components == 1
    }
}
