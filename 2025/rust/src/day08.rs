use std::collections::{BinaryHeap, HashMap};

use util::union_find::UnionFind;

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
