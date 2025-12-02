use std::collections;

pub fn part_one(input: impl AsRef<str>) -> i64 {
    let mat = input
        .as_ref()
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    fn score(
        mat: &Vec<Vec<u32>>,
        visited: &mut collections::HashSet<(usize, usize)>,
        i: i32,
        j: i32,
        next: u32,
    ) -> u32 {
        if i.min(j) < 0 || i >= mat.len() as i32 || j >= mat[0].len() as i32 {
            return 0;
        }

        if mat[i as usize][j as usize] != next {
            return 0;
        }

        let node = (i as usize, j as usize);
        if next >= 9 {
            if !visited.contains(&node) {
                visited.insert(node);
                return 1;
            } else {
                return 0;
            }
        }

        let mut s = 0;
        for (x, y) in [(i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)] {
            s += score(mat, visited, x, y, next + 1);
        }
        s
    }

    let mut res = 0_u32;
    let mut visited: collections::HashSet<(usize, usize)> = collections::HashSet::new();
    for i in 0..mat.len() {
        for j in 0..mat[0].len() {
            visited.clear();
            res += score(&mat, &mut visited, i as i32, j as i32, 0);
        }
    }

    res as i64
}

pub fn part_two(input: impl AsRef<str>) -> i64 {
    let mat = input
        .as_ref()
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    fn score(mat: &Vec<Vec<u32>>, i: i32, j: i32, next: u32) -> u32 {
        if i.min(j) < 0 || i >= mat.len() as i32 || j >= mat[0].len() as i32 {
            return 0;
        }

        if mat[i as usize][j as usize] != next {
            return 0;
        }

        let node = (i as usize, j as usize);
        if next >= 9 {
            return 1;
        }

        let mut s = 0;
        for (x, y) in [(i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)] {
            s += score(mat, x, y, next + 1);
        }
        s
    }

    let mut res = 0_u32;
    for i in 0..mat.len() {
        for j in 0..mat[0].len() {
            res += score(&mat, i as i32, j as i32, 0);
        }
    }

    res as i64
}
