pub const INPUT_DIR: &str = "./2024/questions";

pub fn get_input_for_day(day: u32) -> String {
    std::fs::read_to_string(format!("{}/{:0>2}.txt", INPUT_DIR, day)).unwrap()
}

pub fn get_test_input(day: u32) -> String {
    std::fs::read_to_string(format!("{}/{:0>2}.test.txt", INPUT_DIR, day)).unwrap()
}

pub fn print_mat(mat: &Vec<Vec<char>>) {
  for row in mat {
    for ch in row {
      print!("{ch}");
    }
    println!();
  }
}

pub fn parse_mat(input: &str) -> Vec<Vec<char>> {
  input.lines().map(|line| line.chars().collect()).collect()
}