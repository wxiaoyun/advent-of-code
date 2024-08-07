use num_integer::Integer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    part_one()?;
    part_two()?;

    Ok(())
}

fn part_one() -> Result<(), Box<dyn std::error::Error>> {
    let inputs = std::fs::read_to_string("../input.txt")?;

    let mut inputs = inputs.lines().map(|l| l.trim()).filter(|l| !l.is_empty());
    let insts = inputs.next().unwrap().chars().collect::<Vec<_>>();
    let adj_list = inputs
        .map(|l| {
            let mut eql_split = l.split("=").map(|l| l.trim());
            let from = eql_split.next().unwrap().to_owned();
            let binding = eql_split.next().unwrap().replace("(", "").replace(")", "");
            let mut tos = binding.split(", ").map(|s| s.to_owned());

            (from, (tos.next().unwrap(), tos.next().unwrap()))
        })
        .collect::<std::collections::HashMap<_, _>>();

    let mut steps = 0;
    let mut current = &"AAA".to_string();

    while current != &"ZZZ".to_string() {
        let inst = insts[steps % insts.len()];
        let (left, right) = adj_list.get(current).unwrap();

        current = match inst {
            'L' => left,
            'R' => right,
            c => panic!("Invalid instruction: {}", c),
        };
        steps += 1;
    }

    println!("Steps: {}", steps);

    Ok(())
}

fn part_two() -> Result<(), Box<dyn std::error::Error>> {
    let inputs = std::fs::read_to_string("../input.txt")?;

    let mut inputs = inputs.lines().map(|l| l.trim()).filter(|l| !l.is_empty());
    let insts = inputs.next().unwrap().chars().collect::<Vec<_>>();
    let adj_list = inputs
        .map(|l| {
            let mut eql_split = l.split("=").map(|l| l.trim());
            let from = eql_split.next().unwrap().to_owned();
            let binding = eql_split.next().unwrap().replace("(", "").replace(")", "");
            let mut tos = binding.split(", ").map(|s| s.to_owned());

            (from, (tos.next().unwrap(), tos.next().unwrap()))
        })
        .collect::<std::collections::HashMap<_, _>>();

    let steps = adj_list
        .keys()
        .filter(|&k| k.ends_with("A"))
        .map(|k| {
            let mut current = k;
            let mut steps = 0;

            while !current.ends_with("Z") {
                let inst = insts[steps % insts.len()];
                let (left, right) = adj_list.get(current).unwrap();

                current = match inst {
                    'L' => left,
                    'R' => right,
                    c => panic!("Invalid instruction: {}", c),
                };
                steps += 1;
            }

            steps
        })
        .fold(1, |accum, steps| accum.lcm(&steps));

    println!("Steps: {}", steps);

    Ok(())
}
