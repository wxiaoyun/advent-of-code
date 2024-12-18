use std::collections;

use crate::{get_input_for_day, get_test_input};

const START: char = 'S';
const END: char = 'E';
const WALL: char = '#';
const UP: u8 = 0;
const RIGHT: u8 = 1;
const DOWN: u8 = 2;
const LEFT: u8 = 3;
const DIRECTIONS: [u8; 4] = [UP, RIGHT, DOWN, LEFT];

pub fn part_one() {
    let mat = get_input_for_day(16)
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut si = 0;
    let mut sj = 0;
    let mut ei = 0;
    let mut ej = 0;

    for (i, row) in mat.iter().enumerate() {
        for (j, &ch) in row.iter().enumerate() {
            match ch {
                START => {
                    si = i;
                    sj = j;
                }
                END => {
                    ei = i;
                    ej = j;
                }
                _ => {}
            }
        }
    }

    fn turn(dir: u8) -> [u8; 2] {
        let mut res = [0; 2];

        let idx = match dir {
            UP => 0,
            RIGHT => 1,
            DOWN => 2,
            LEFT => 3,
            _ => unreachable!(),
        };

        res[0] = DIRECTIONS[(idx + 4 + 1) % 4];
        res[1] = DIRECTIONS[(idx + 4 - 1) % 4];
        res
    }

    fn next(i: usize, j: usize, dir: u8) -> (usize, usize) {
        match dir {
            UP => (i - 1, j),
            RIGHT => (i, j + 1),
            DOWN => (i + 1, j),
            LEFT => (i, j - 1),
            _ => unreachable!(),
        }
    }

    fn dijkstra(
        mat: &Vec<Vec<char>>,
        start_i: usize,
        start_j: usize,
        end_i: usize,
        end_j: usize,
    ) -> u64 {
        let mut visited: collections::HashSet<(usize, usize, u8)> = collections::HashSet::new();
        let mut pq: collections::BinaryHeap<(i64, usize, usize, u8)> =
            collections::BinaryHeap::from([(0, start_i, start_j, RIGHT)]);

        while !pq.is_empty() {
            let (w, i, j, dir) = pq.pop().unwrap();
            
            let node = (i, j, dir);
            if visited.contains(&node) {
              continue;
            }
            visited.insert(node);
            

            if i == end_i && j == end_j {
                return (-w) as u64;
            }

            if mat[i][j] == WALL {
                continue;
            }

            let (ii, jj) = next(i, j, dir);
            pq.push((w - 1, ii, jj, dir));

            for d in turn(dir) {
                pq.push((w - 1000, i, j, d));
            }
        }

        unreachable!()
    }

    let res = dijkstra(&mat, si, sj, ei, ej);
    println!("{}", res);
}

pub fn part_two() {}
