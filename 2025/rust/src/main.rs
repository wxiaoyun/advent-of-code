#![feature(likely_unlikely)]

mod day01;
mod day02;
mod day03;

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
        _ => panic!("Day {}, part {} not implemented", args.day, args.part),
    };

    println!("Result: {}", result);
}
