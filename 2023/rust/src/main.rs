#![feature(btree_cursors)]
#![allow(unused)]

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod prelude;

use prelude::*;

fn main() {
    let args = util::parse_args();

    match (args.day, args.part) {
        (1, 1) => day01::part_one(),
        (1, 2) => day01::part_two(),
        (2, 1) => day02::part_one(),
        (2, 2) => day02::part_two(),
        (3, 1) => day03::part_one(),
        (3, 2) => day03::part_two(),
        (4, 1) => day04::part_one(),
        (4, 2) => day04::part_two(),
        (5, 1) => day05::part_one(),
        (5, 2) => day05::part_two(),
        (6, 1) => day06::part_one(),
        (6, 2) => day06::part_two(),
        (7, 1) => day07::part_one(),
        (7, 2) => day07::part_two(),
        (8, 1) => day08::part_one(),
        (8, 2) => day08::part_two(),
        (9, 1) => day09::part_one(),
        (9, 2) => day09::part_two(),
        (10, 1) => day10::part_one(),
        (11, 1) => day11::part_one(),
        (11, 2) => day11::part_two(),
        (12, 1) => day12::part_one(),
        (12, 2) => day12::part_two(),
        (13, 1) => day13::part_one(),
        (13, 2) => day13::part_two(),
        (14, 1) => day14::part_one(),
        (14, 2) => day14::part_two(),
        (15, 1) => day15::part_one(),
        (15, 2) => day15::part_two(),
        _ => panic!("Day {}, part {} not implemented", args.day, args.part),
    };
}
