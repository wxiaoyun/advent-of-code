use clap::Parser;

#[derive(Parser, Debug)]
#[command(version)]
pub struct Args {
    #[arg(short, long)]
    pub year: u32,
    #[arg(short, long)]
    pub day: u8,
    #[arg(short, long, default_value = "1")]
    pub part: u8,
}

pub fn parse_args() -> Args {
    Args::parse()
}

pub const INPUT_FOLDER_NAME: &str = "input";

pub fn read_input(year: impl Into<i64>, day: impl Into<i64>) -> String {
    std::fs::read_to_string(format!(
        "{:?}/{INPUT_FOLDER_NAME}/{:?}.txt",
        year.into(),
        day.into()
    ))
    .unwrap()
}
