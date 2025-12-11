#![feature(likely_unlikely)]
#![allow(clippy::ptr_arg, clippy::needless_range_loop)]

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

fn main() {
    let args = util::parse_args();
    let input = util::read_input_from_stdin();

    let result: i64 = match (args.day, args.part) {
        (1, 1) => day01::part_one(input),
        (1, 2) => day01::part_two(input),
        (2, 1) => day02::part_one(input),
        (2, 2) => day02::part_two(input),
        (3, 1) => day03::part_one(input),
        (3, 2) => day03::part_two(input),
        (4, 1) => day04::part_one(input),
        (4, 2) => day04::part_two(input),
        (5, 1) => day05::part_one(input),
        (5, 2) => day05::part_two(input),
        (6, 1) => day06::part_one(input),
        (6, 2) => day06::part_two(input),
        (7, 1) => day07::part_one(input),
        (7, 2) => day07::part_two(input),
        (8, 1) => day08::part_one(input),
        (8, 2) => day08::part_two(input),
        (9, 1) => day09::part_one(input),
        (9, 2) => day09::part_two(input),
        (10, 1) => day10::part_one(input),
        (10, 2) => day10::part_two(input),
        (11, 1) => day11::part_one(input),
        (11, 2) => day11::part_two(input),
        _ => panic!("Day {}, part {} not implemented", args.day, args.part),
    };

    println!("Result: {}", result);
}
