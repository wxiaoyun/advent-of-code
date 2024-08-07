pub const INPUT_DIR: &str = "../questions";

pub fn get_input_for_day(day: u32) -> String {
    std::fs::read_to_string(format!("{}/{:0>2}.txt", INPUT_DIR, day)).unwrap()
}
