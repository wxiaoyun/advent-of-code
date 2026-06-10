use std::collections::{HashMap, VecDeque};

const START: char = 'S';
const PLOT: char = '.';
const ROCK: char = '#';

const MAX_STEPS: usize = 64;

pub fn part_one(input: impl AsRef<str>) -> i64 {
    let (grid, (sr, sc)) = parse_input(input);
    let (m, n) = (grid.len() as i64, grid[0].len() as i64);

    let mut even_odd = HashMap::new();
    let mut q: VecDeque<(i64, i64)> = VecDeque::new();
    q.push_back((sr as i64, sc as i64));
    let mut n_steps = 0;
    while !q.is_empty() && n_steps <= MAX_STEPS {
        let len = q.len();

        for _ in 0..len {
            let node = q.pop_front().unwrap();
            if even_odd.contains_key(&node) {
                continue;
            }
            even_odd.insert(node.clone(), (n_steps & 1) as i64 ^ 1);

            let (sr, sc) = node;
            [(sr - 1, sc), (sr + 1, sc), (sr, sc - 1), (sr, sc + 1)]
                .into_iter()
                .for_each(|(r, c)| {
                    if r.min(c) < 0 || r >= m || c >= n {
                        return;
                    }
                    if grid[r as usize][c as usize] == ROCK {
                        return;
                    }
                    q.push_back((r, c));
                });
        }

        n_steps += 1;
    }

    even_odd.values().copied().sum::<i64>()
}

pub fn part_two(input: impl AsRef<str>) -> i64 {
    0
}

fn parse_input(input: impl AsRef<str>) -> (Vec<Vec<char>>, (usize, usize)) {
    let mut start: Option<(usize, usize)> = None;
    let grid = input
        .as_ref()
        .lines()
        .enumerate()
        .map(|(r, l)| {
            let chars = l.chars();
            chars.clone().enumerate().for_each(|(c, ch)| {
                if ch == START {
                    start = Some((r, c));
                }
            });
            chars.collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let Some(start) = start else {
        panic!("Starting coordinate not found");
    };

    (grid, start)
}
