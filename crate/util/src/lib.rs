use clap::Parser;

#[derive(Parser, Debug)]
#[command(version)]
pub struct Args {
    #[arg(short, long)]
    pub day: u8,
    #[arg(short, long, default_value = "1")]
    pub part: u8,
}

pub fn parse_args() -> Args {
    Args::parse()
}

pub const INPUT_FOLDER_NAME: &str = "input";

pub fn read_input(year: impl Into<u32>, day: impl Into<u8>) -> String {
    std::fs::read_to_string(format!(
        "{:?}/{INPUT_FOLDER_NAME}/{:?}.txt",
        year.into(),
        day.into()
    ))
    .unwrap()
}

pub fn read_test_input(year: impl Into<u32>, day: impl Into<u8>) -> String {
    std::fs::read_to_string(format!(
        "{:?}/{INPUT_FOLDER_NAME}/{:?}_test.txt",
        year.into(),
        day.into()
    ))
    .unwrap()
}
