mod day01;

fn main() {
    let args = util::parse_args();
    let input = util::read_input(2025, args.day);

    let result = match (args.day, args.part) {
        (1, 1) => day01::part_one(input),
        (1, 2) => day01::part_two(input),
        _ => panic!("Day {}, part {} not implemented", args.day, args.part),
    };

    println!("Result: {}", result);
}
