fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("../input.txt")?;

    let res = input
        .lines()
        .map(|line| {
            let parts = line.split(": ").collect::<Vec<_>>();
            let parts = parts[1].split("|").collect::<Vec<_>>();
            let mut count = 0;

            let winning_nums = parts[0]
                .split_ascii_whitespace()
                .map(|s| s.trim())
                .flat_map(|x| x.parse::<u32>())
                .collect::<std::collections::HashSet<_>>();

            parts[1]
                .split_ascii_whitespace()
                .map(|s| s.trim())
                .flat_map(|x| x.parse::<u32>())
                .for_each(|x| {
                    if winning_nums.contains(&x) {
                        count += 1;
                    }
                });

            if count == 0 {
                0
            } else {
                1 << (count - 1)
            }
        })
        .sum::<u64>();

    println!("Sum of points {}", res);

    Ok(())
}
