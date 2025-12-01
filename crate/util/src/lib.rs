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
