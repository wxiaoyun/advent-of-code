mod day01;

fn main() {
    let args = util::parse_args();

    match (args.day, args.part) {
        (1, 1) => day01::part_one(),
        (1, 2) => day01::part_two(),
        _ => panic!("Day {}, part {} not implemented", args.day, args.part),
    }
}