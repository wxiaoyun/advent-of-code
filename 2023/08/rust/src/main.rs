fn main() -> Result<(), Box<dyn std::error::Error>> {
    part_one()?;

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
            _ => panic!("Invalid instruction"),
        };
        steps += 1;
    }

    println!("Steps: {}", steps);

    Ok(())
}
