use crate::get_input_for_day;

pub fn part_one() -> Result<(), Box<dyn std::error::Error>> {
    let inputs = std::fs::read_to_string(get_input_for_day(6))?;

    let mut inputs = inputs.lines();
    let time_inputs = inputs
        .next()
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .split_ascii_whitespace()
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u64>().unwrap());
    let record_distance_inputs = inputs
        .next()
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .split_ascii_whitespace()
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u64>().unwrap());
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

pub fn part_two() -> Result<(), Box<dyn std::error::Error>> {
    let inputs = std::fs::read_to_string(get_input_for_day(6))?;

    let mut inputs = inputs.lines();

    let total_time = inputs
        .next()
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .split_ascii_whitespace()
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("")
        .parse::<u64>()?;
    let record_dist = inputs
        .next()
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .split_ascii_whitespace()
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("")
        .parse::<u64>()?;
    
    let mut left = -1;
    for t in 1..=total_time {
        let speed = t;
        let distance = speed * (total_time - t);
        if distance > record_dist {
            left = t as i64;
            break;
        }
    }

    let mut right = -1;
    for t in (1..=total_time).rev() {
        let speed = t;
        let distance = speed * (total_time - t);
        if distance > record_dist {
            right = t as i64;
            break;
        }
    }

    let res = right - left + 1;
    println!("Number of ways: {}", res);

    Ok(())
}
