#![feature(likely_unlikely)]

mod day01;
mod day02;

fn main() {
    let args = util::parse_args();
    let input = util::read_input(args.year, args.day);

    let result = match (args.day, args.part) {
        (1, 1) => day01::part_one(input),
        (1, 2) => day01::part_two(input),
        (2, 1) => day02::part_one(input),
        (2, 2) => day02::part_two(input),
        _ => panic!("Day {}, part {} not implemented", args.day, args.part),
    };

    println!("Result: {}", result);
}
