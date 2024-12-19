use std::collections;

use crate::{get_input_for_day, get_test_input, print_mat};

const GRID_SIZE: usize = 71;
const BYTES_TAKEN: usize = 1024;

pub fn part_one() {
    let bytes = get_input_for_day(18)
        .lines()
        .map(|l| {
            let mut nums = l.split(',').map(|n| n.parse::<u64>().unwrap());
            (nums.next().unwrap(), nums.next().unwrap())
        })
        .take(BYTES_TAKEN)
        .collect::<collections::HashSet<_>>();

    let mut visited = collections::HashSet::new();
    let mut q = collections::VecDeque::from([(0, 0, 0)]);

    while let Some((i, j, s)) = q.pop_front() {
        if bytes.contains(&(i as u64, j as u64)) {
            continue;
        }

        if i == GRID_SIZE - 1 && j == GRID_SIZE - 1 {
            println!("Part one: {}", s);
            break;
        }

        if visited.contains(&(i, j)) {
            continue;
        }
        visited.insert((i, j));

        for (di, dj) in [(0, 1), (1, 0), (0, -1), (-1, 0)].iter() {
            let ni = i as i32 + di;
            let nj = j as i32 + dj;
            if ni >= 0 && ni < GRID_SIZE as i32 && nj >= 0 && nj < GRID_SIZE as i32 {
                q.push_back((ni as usize, nj as usize, s + 1));
            }
        }
    }
}

pub fn part_two() {
    let bytes = get_input_for_day(18)
        .lines()
        .map(|l| {
            let mut nums = l.split(',').map(|n| n.parse::<u64>().unwrap());
            (nums.next().unwrap(), nums.next().unwrap())
        })
        .collect::<Vec<_>>();

    fn bfs(bytes: &collections::HashSet<(u64, u64)>) -> bool {
        let mut visited = collections::HashSet::new();
        let mut q = collections::VecDeque::from([(0, 0)]);

        while let Some((i, j)) = q.pop_front() {
            if bytes.contains(&(i as u64, j as u64)) {
                continue;
            }

            if i == GRID_SIZE - 1 && j == GRID_SIZE - 1 {
                return true;
            }

            if visited.contains(&(i, j)) {
                continue;
            }
            visited.insert((i, j));

            for (di, dj) in [(0, 1), (1, 0), (0, -1), (-1, 0)].iter() {
                let ni = i as i32 + di;
                let nj = j as i32 + dj;
                if ni >= 0 && ni < GRID_SIZE as i32 && nj >= 0 && nj < GRID_SIZE as i32 {
                    q.push_back((ni as usize, nj as usize));
                }
            }
        }

        false
    }

    let mut l = 0;
    let mut r = bytes.len();

    // Binary search for the first byte that will prevent the exit from being reachable
    while l < r {
        let m = (l + r) / 2;
        let mut bytes_set = bytes
            .clone()
            .into_iter()
            .take(m)
            .collect::<collections::HashSet<_>>();

        if bfs(&bytes_set) {
            l = m + 1;
        } else {
            r = m;
        }
    }

    println!("Part two: {}, {:?}", l-1, bytes[l-1]);
}
