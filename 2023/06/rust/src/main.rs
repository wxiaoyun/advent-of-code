fn main() -> Result<(), Box<dyn std::error::Error>> {
    part_one()?;

    Ok(())
}

fn part_one() -> Result<(), Box<dyn std::error::Error>> {
    let inputs = std::fs::read_to_string("../input.txt")?;

    let mut inputs = inputs.lines();
    let time_inputs = inputs
        .next()
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .split_ascii_whitespace()
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u32>().unwrap());
    let record_distance_inputs = inputs
        .next()
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .split_ascii_whitespace()
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u32>().unwrap());
    let time_dist = time_inputs.zip(record_distance_inputs).collect::<Vec<_>>();

    let mut res = 1;
    for (total_time, record_dist) in time_dist {
        let mut tmp = 0;
        for t in 1..=total_time {
            let speed = t;
            let distance = speed * (total_time - t);
            if distance > record_dist {
                tmp += 1;
            }
        }
        res *= tmp;
    }

    println!("Number of ways: {}", res);

    Ok(())
}
