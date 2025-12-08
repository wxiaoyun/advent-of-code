use std::collections::{BinaryHeap, HashMap};

fn parse_input(input: impl AsRef<str>) -> Vec<Vec<i64>> {
    input
        .as_ref()
        .lines()
        .map(|l| {
            l.split(',')
                .take(3)
                .map(|s| s.parse::<i64>())
                .collect::<Result<_, _>>()
        })
        .collect::<Result<_, _>>()
        .unwrap()
}

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    components: usize,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        let mut rank = Vec::new();
        rank.resize(size, 1);
        Self {
            parent: (0..size).collect(),
            rank,
            components: 20,
        }
    }

    fn find(&mut self, a: usize) -> usize {
        let ap = self.parent[a];
        if a == ap {
            return a;
        }

        let app = self.find(ap);
        self.parent[a] = app;
        app
    }

    fn union(&mut self, a: usize, b: usize) {
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

    fn is_connected(&self) -> bool {
        self.components == 1
    }
}

static mut CONNECTIONS: usize = 1000;

pub fn part_one(input: impl AsRef<str>) -> i64 {
    let coords = parse_input(input);

    let mut heap = BinaryHeap::with_capacity(coords.len() * (coords.len() - 1) / 2);
    for i in 0..coords.len() {
        let (ix, iy, iz) = (coords[i][0], coords[i][1], coords[i][2]);
        for j in (i + 1)..coords.len() {
            let (jx, jy, jz) = (coords[j][0], coords[j][1], coords[j][2]);
            let dist_sq = (ix - jx).pow(2) + (iy - jy).pow(2) + (iz - jz).pow(2);
            heap.push((-dist_sq, i, j));
        }
    }

    let mut uf = UnionFind::new(coords.len());
    let mut connection_count = 0;
    while let Some((_, i, j)) = heap.pop() {
        if connection_count >= unsafe { CONNECTIONS } {
            break;
        }
        connection_count += 1;

        if uf.find(i) == uf.find(j) {
            continue;
        }

        uf.union(i, j);
    }

    let mut component_size = HashMap::with_capacity(coords.len());
    for i in 0..coords.len() {
        let ip = uf.find(i);
        let isize = uf.rank[ip];
        component_size.insert(ip, isize as i64);
    }

    component_size
        .values()
        .fold(BinaryHeap::with_capacity(3), |mut hp, &size| {
            hp.push(-size);
            if hp.len() > 3 {
                hp.pop();
            }
            hp
        })
        .into_iter()
        .fold(1, |acc, size| acc * (-size))
}

pub fn part_two(input: impl AsRef<str>) -> i64 {
    let coords = parse_input(input);

    let mut heap = BinaryHeap::with_capacity(coords.len() * (coords.len() - 1) / 2);
    for i in 0..coords.len() {
        let (ix, iy, iz) = (coords[i][0], coords[i][1], coords[i][2]);
        for j in (i + 1)..coords.len() {
            let (jx, jy, jz) = (coords[j][0], coords[j][1], coords[j][2]);
            let dist_sq = (ix - jx).pow(2) + (iy - jy).pow(2) + (iz - jz).pow(2);
            heap.push((-dist_sq, i, j));
        }
    }

    let mut uf = UnionFind::new(coords.len());
    while let Some((_, i, j)) = heap.pop() {
        if uf.find(i) == uf.find(j) {
            continue;
        }

        uf.union(i, j);

        if uf.is_connected() {
            return coords[i][0] * coords[j][0];
        }
    }

    unreachable!()
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use crate::day08::CONNECTIONS;

    const TEST_INPUT: &str = indoc! {"
        162,817,812
        57,618,57
        906,360,560
        592,479,940
        352,342,300
        466,668,158
        542,29,236
        431,825,988
        739,650,466
        52,470,668
        216,146,977
        819,987,18
        117,168,530
        805,96,715
        346,949,466
        970,615,88
        941,993,340
        862,61,35
        984,92,344
        425,690,689
    "};

    #[test]
    fn test_part1() {
        unsafe { CONNECTIONS = 10 };
        assert_eq!(40, super::part_one(TEST_INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(25272, super::part_two(TEST_INPUT));
    }
}
