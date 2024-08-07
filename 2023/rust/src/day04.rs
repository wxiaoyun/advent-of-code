use crate::get_input_for_day;

pub fn part_one() -> Result<(), Box<dyn std::error::Error>> {
    let input = get_input_for_day(4);

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

    println!("(Part one) Sum of points {}", res);

    Ok(())
}

pub fn part_two() -> Result<(), Box<dyn std::error::Error>> {
    let input = get_input_for_day(4);

    let input = input
        .lines()
        .map(|l| {
            let parts = l.split(": ").collect::<Vec<_>>();

            let card_parts = parts[0]
                .split(" ")
                .filter(|&s| !s.is_empty())
                .collect::<Vec<_>>();
            let card_num = card_parts[1].parse::<u32>().unwrap();

            let parts = parts[1].split("|").collect::<Vec<_>>();
            let winning_nums = parts[0]
                .split_ascii_whitespace()
                .map(|s| s.trim())
                .flat_map(|x| x.parse::<u32>())
                .collect::<std::collections::HashSet<_>>();
            let current_nums = parts[1]
                .split_ascii_whitespace()
                .map(|s| s.trim())
                .flat_map(|x| x.parse::<u32>())
                .collect::<Vec<_>>();

            (card_num, (winning_nums, current_nums))
        })
        .collect::<std::collections::HashMap<_, _>>();

    fn process_card(
        cache: &mut std::collections::HashMap<u32, u32>,
        input: &std::collections::HashMap<u32, (std::collections::HashSet<u32>, Vec<u32>)>,
        card: u32,
    ) -> u32 {
        if cache.contains_key(&card) {
            return cache[&card];
        }

        let mut count = 0;
        let (winning_nums, current_nums) = &input[&card];
        for x in current_nums {
            if winning_nums.contains(x) {
                count += 1;
            }
        }

        let mut res = 1;
        for i in (card + 1)..=(card + count) {
            res += process_card(cache, input, i);
        }

        cache.insert(card, res);
        res
    }

    let mut res = 0;
    let mut cache = std::collections::HashMap::new();
    for card in input.keys() {
        res += process_card(&mut cache, &input, *card);
    }

    println!("(Part two) Sum of points {}", res);

    Ok(())
}
