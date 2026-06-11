use std::collections::{HashMap, VecDeque};

const START: char = 'S';
const PLOT: char = '.';
const ROCK: char = '#';

const MAX_STEPS_P1: usize = 64;

pub fn part_one(input: impl AsRef<str>) -> i64 {
    let (grid, (sr, sc)) = parse_input(input);
    let (m, n) = (grid.len() as i64, grid[0].len() as i64);

    let mut even_odd = HashMap::new();
    let mut q: VecDeque<(i64, i64)> = VecDeque::new();
    q.push_back((sr as i64, sc as i64));
    let mut n_steps = 0;
    while !q.is_empty() && n_steps <= MAX_STEPS_P1 {
        let len = q.len();

        for _ in 0..len {
            let node = q.pop_front().unwrap();
            if even_odd.contains_key(&node) {
                continue;
            }
            even_odd.insert(
                node.clone(),
                if n_steps % 2 == MAX_STEPS_P1 % 2 {
                    1
                } else {
                    0
                },
            );

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

const MAX_STEPS_P2: usize = 26501365;
const GRID_DIM: usize = 131;
const GRID_REM: usize = MAX_STEPS_P2 % GRID_DIM;
const N_GRID: usize = MAX_STEPS_P2 / GRID_DIM;

pub fn part_two(input: impl AsRef<str>) -> i64 {
    let (grid, (sr, sc)) = parse_input(input);

    let y0 = count_with_start(&grid, sr, sc, GRID_REM);
    let y1 = count_with_start(&grid, sr, sc, GRID_REM + GRID_DIM);
    let y2 = count_with_start(&grid, sr, sc, GRID_REM + GRID_DIM * 2);

    let c = y0;
    let a = (y2 - 2 * y1 + y0) / 2;
    let b = y1 - y0 - a;

    let x = N_GRID as i64;
    let total_plots = a * x * x + b * x + c;

    total_plots
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

fn count_with_start(grid: &Vec<Vec<char>>, sr: usize, sc: usize, max_steps: usize) -> i64 {
    let (m, n) = (grid.len() as i64, grid[0].len() as i64);
    let mut even_odd = HashMap::new();
    let mut q: VecDeque<(i64, i64)> = VecDeque::new();
    q.push_back((sr as i64, sc as i64));
    let mut n_steps = 0;
    while !q.is_empty() && n_steps <= max_steps {
        let len = q.len();

        for _ in 0..len {
            let node = q.pop_front().unwrap();
            if even_odd.contains_key(&node) {
                continue;
            }
            even_odd.insert(
                node.clone(),
                if n_steps % 2 == max_steps % 2 { 1 } else { 0 },
            );

            let (sr, sc) = node;
            [(sr - 1, sc), (sr + 1, sc), (sr, sc - 1), (sr, sc + 1)]
                .into_iter()
                .for_each(|(r, c)| {
                    if grid[r.rem_euclid(m) as usize][c.rem_euclid(n) as usize] == ROCK {
                        return;
                    }
                    q.push_back((r, c));
                });
        }

        n_steps += 1;
    }

    even_odd.values().copied().sum::<i64>()
}
