use crate::get_input_for_day;

const RED_KEY: &str = "red";
const MAX_RED: u32 = 12;
const GREEN_KEY: &str = "green";
const MAX_GREEN: u32 = 13;
const BLUE_KEY: &str = "blue";
const MAX_BLUE: u32 = 14;

fn part_one() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(get_input_for_day(2))?;

    let sum = input
        .split("\n")
        .map(|game| {
            let temp = game.split(": ").collect::<Vec<_>>();

            let id = temp[0].split(" ").collect::<Vec<_>>()[1]
                .parse::<u32>()
                .unwrap();

            let sets = temp[1]
                .split("; ")
                .map(|set| {
                    set.split(", ")
                        .map(|pair| {
                            let pair = pair.split(" ").collect::<Vec<_>>();
                            let count = pair[0];
                            let color = pair[1];
                            let count = count.parse::<u32>().unwrap();

                            (color, count)
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<Vec<_>>>();

            let mut tabulation = std::collections::HashMap::new();
            for set in sets {
                for (color, count) in set {
                    let current = tabulation.get(color).unwrap_or(&0);
                    tabulation.insert(color, (*current).max(count));
                }
            }
            (id, tabulation)
        })
        .filter(|(_, tabulation)| {
            let red = tabulation.get(RED_KEY).unwrap_or(&0);
            if *red > MAX_RED {
                return false;
            }

            let green = tabulation.get(GREEN_KEY).unwrap_or(&0);
            if *green > MAX_GREEN {
                return false;
            }

            let blue = tabulation.get(BLUE_KEY).unwrap_or(&0);
            if *blue > MAX_BLUE {
                return false;
            }

            true
        })
        .map(|(id, _)| id)
        .sum::<u32>();

    println!("Sum of IDs: {}", sum);

    Ok(())
}
