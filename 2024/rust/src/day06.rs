use std::collections;

pub fn part_one(input: impl AsRef<str>) -> i64 {
    let map = input
        .as_ref()
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let nrow = map.len();
    let ncol = map[0].len();

    let (mut i, mut j) = (0_usize, 0_usize);
    'outer: for (r, row) in map.iter().enumerate() {
        for (c, &ch) in row.iter().enumerate() {
            if ch == '^' {
                i = r;
                j = c;
                break 'outer;
            }
        }
    }

    let up = (-1, 0);
    let down = (1, 0);
    let left = (0, -1);
    let right = (0, 1);
    let directions = [up, right, down, left];
    let mut dir_idx = 0;
    let mut dir = up;
    let mut visited = collections::HashSet::from([(i, j)]);
    let mut steps = 1;

    loop {
        let (dy, dx) = dir;
        let nr = i as i32 + dy;
        let nc = j as i32 + dx;

        // Reached edge
        if nr.min(nc) < 0 || nr >= nrow as i32 || nc >= ncol as i32 {
            break;
        }

        // Turn right if facing obstacle
        if map[nr as usize][nc as usize] == '#' {
            dir_idx = (dir_idx + 1) % directions.len();
            dir = directions[dir_idx];
            continue;
        }

        i = nr as usize;
        j = nc as usize;
        if !visited.contains(&(i, j)) {
            steps += 1;
            visited.insert((i, j));
        }
    }

    steps as i64
}

pub fn part_two(input: impl AsRef<str>) -> i64 {
    let mut map = input
        .as_ref()
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let nrow = map.len();
    let ncol = map[0].len();

    let (mut i, mut j) = (0_usize, 0_usize);
    'outer: for (r, row) in map.iter().enumerate() {
        for (c, &ch) in row.iter().enumerate() {
            if ch == '^' {
                i = r;
                j = c;
                break 'outer;
            }
        }
    }

    fn check_loop(mat: &Vec<Vec<char>>, mut i: usize, mut j: usize) -> bool {
        let up = (-1, 0);
        let down = (1, 0);
        let left = (0, -1);
        let right = (0, 1);
        let directions = [up, right, down, left];
        let mut dir_idx = 0;
        let mut dir = up;
        let mut visited = collections::HashSet::new();

        loop {
            if visited.contains(&(i, j, dir)) {
                return true; // we have revisited the same location with the same direction
            }
            visited.insert((i, j, dir));

            let (dy, dx) = dir;
            let nr = i as i32 + dy;
            let nc = j as i32 + dx;

            // Reached edge, exited matrix
            if nr.min(nc) < 0 || nr >= mat.len() as i32 || nc >= mat[0].len() as i32 {
                return false;
            }

            // Turn right if facing obstacle
            if mat[nr as usize][nc as usize] == '#' {
                dir_idx = (dir_idx + 1) % directions.len();
                dir = directions[dir_idx];
                continue;
            }

            i = nr as usize;
            j = nc as usize;
        }
    }

    let mut loops = 0;
    for r in 0..nrow {
        for c in 0..ncol {
            if map[r][c] != '#' && !(r == i && c == j) {
                map[r][c] = '#';
                if check_loop(&map, i, j) {
                    loops += 1;
                }
                map[r][c] = '.';
            }
        }
    }

    loops as i64
}
