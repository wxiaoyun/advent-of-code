pub const INPUT_DIR: &str = "./2023/questions";

pub type Result<Ok = ()> = std::result::Result<Ok, Box<dyn std::error::Error>>;

pub fn get_input_for_day(day: u32) -> String {
    std::fs::read_to_string(format!("{}/{:0>2}.txt", INPUT_DIR, day)).unwrap()
}

pub fn get_test_input(file_name: &str) -> String {
    std::fs::read_to_string(format!("{}/{}", INPUT_DIR, file_name)).unwrap()
}
