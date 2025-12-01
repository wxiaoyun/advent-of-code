use std::{path::PathBuf, time::Duration};

use anyhow::Result;
use clap::Parser;
use futures::TryFutureExt;
use reqwest::{
    Client,
    header::{HeaderMap, HeaderName, HeaderValue},
};
use tokio::sync::OnceCell;

#[derive(Parser, Debug)]
#[command(version)]
pub struct Args {
    #[arg(short, long)]
    pub year: u32,
    #[arg(
        short,
        long,
        help = "The day to scrape. If not provided, all days will be scraped."
    )]
    pub day: Option<u8>,
    #[arg(short, long, help = "Advent of Code session token")]
    pub session_token: String,
    #[arg(short, long, default_value = "3")]
    pub max_retries: u8,
    #[arg(short = 'D', long, default_value = "5")]
    pub delay: u8,
    #[arg(short, long, help = "The directory to save the scraped data.")]
    pub output_dir: Option<PathBuf>,
}

static REQWEST_CLIENT: OnceCell<Client> = OnceCell::const_new();

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let session_token = args.session_token;
    let max_retries = args.max_retries;
    let delay = args.delay;
    let output_dir = args
        .output_dir
        .unwrap_or(format!("{}/{}", args.year, util::INPUT_FOLDER_NAME).into());

    if tokio::fs::try_exists(&output_dir).await.unwrap_or(false) {
        tokio::fs::create_dir_all(&output_dir).await?;
    }

    REQWEST_CLIENT
        .get_or_init(|| async move {
            let headers = HeaderMap::from_iter(vec![(
                HeaderName::from_static("cookie"),
                HeaderValue::from_str(&format!("session={}", session_token)).unwrap(),
            )]);

            Client::builder().default_headers(headers).build().unwrap()
        })
        .await;

    let days = args
        .day
        .map(|day| vec![day])
        .unwrap_or_else(|| (1..=25).collect());

    let mut js = tokio::task::JoinSet::new();
    days.into_iter().for_each(|day| {
        js.spawn(scrape_day(
            args.year,
            day,
            max_retries,
            delay,
            output_dir.clone(),
        ));
    });

    js.join_all().await;
    Ok(())
}

async fn scrape_day(
    year: u32,
    day: u8,
    max_retries: u8,
    delay: u8,
    output_dir: PathBuf,
) -> Result<()> {
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);

    let fetch_input = || async {
        REQWEST_CLIENT
            .get()
            .unwrap()
            .get(&url)
            .send()
            .and_then(|response| response.text())
            .await
    };

    let mut retries = 0;
    let body = loop {
        let res = fetch_input().await;
        let e = match res {
            Ok(body) => {
                break body;
            }
            Err(e) => e,
        };

        if e.status().is_some_and(|status| status == 404) {
            return Err(anyhow::Error::msg("No question found for day {day}"));
        };

        if retries >= max_retries {
            return Err(anyhow::Error::from(e));
        }
        retries += 1;
        tokio::time::sleep(Duration::from_secs(delay as u64)).await;
    };

    let path = output_dir.join(format!("{}.txt", day));
    tokio::fs::write(path, body.as_bytes()).await?;

    println!("Saved input for Advent of Code {year}/{day:02}");

    Ok(())
}
