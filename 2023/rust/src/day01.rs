use crate::get_input_for_day;

pub fn part_one() -> Result<(), Box<dyn std::error::Error>> {
    let bytes = std::fs::read(get_input_for_day(1))?;
    let txt: &str = std::str::from_utf8(&bytes)?;

    let calibration_sum = txt
        .lines()
        .map(|l| l.chars().filter(|c| c.is_ascii_digit()).collect())
        .collect::<Vec<Vec<char>>>()
        .into_iter()
        .map(|l| {
            let first_digit = l.first().and_then(|c| c.to_digit(10)).unwrap(); // Safe given input
            let last_digit = l.last().and_then(|c| c.to_digit(10)).unwrap(); // Safe given input
            first_digit * 10 + last_digit
        })
        .sum::<u32>();

    println!("Sum of first and last digits: {}", calibration_sum);
    Ok(())
}
