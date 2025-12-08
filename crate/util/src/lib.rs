use std::io::Read;

pub mod union_find;

pub struct Args {
    pub day: u8,
    pub part: u8,
}

pub fn parse_args() -> Args {
    let mut args = std::env::args();
    let day = args
        .nth(1)
        .and_then(|s| s.parse::<u8>().ok())
        .expect("Positional argument day is required");
    let part = args
        .next()
        .and_then(|s| s.parse::<u8>().ok())
        .expect("Positional argument part is required");

    Args { day, part }
}

pub fn read_input_from_stdin() -> String {
    let mut buf = Vec::new();
    std::io::stdin()
        .read_to_end(&mut buf)
        .expect("Failed to read input from stdin");
    String::from_utf8(buf).expect("Failed to convert input to string")
}
