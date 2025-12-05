use std::{path::PathBuf, str::FromStr, time::Duration};

use anyhow::{Context, Result, anyhow};
use clap::Parser;
use futures::TryFutureExt;
use reqwest::{
    Client,
    header::{HeaderMap, HeaderName, HeaderValue},
};
use tokio::sync::OnceCell;

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    #[arg(short, long)]
    pub year: u32,
    #[arg(short, long, help = "The day/s to scrape.")]
    pub days: IntRangeOption,
    #[arg(short, long, help = "Advent of Code session token")]
    pub session_token: String,
    #[arg(short, long, default_value = "3")]
    pub max_retries: u8,
    #[arg(short = 'D', long, default_value = "5")]
    pub delay: u8,
    #[arg(short, long, help = "The directory to save the scraped data.")]
    pub output_dir: PathBuf,
}

#[derive(Clone, Debug)]
enum IntRangeOption {
    Single(i64),
    Range(i64, i64),
    Selection(Vec<i64>),
}

const RANGE_FORMAT: &str = "Expected range format of \\d+-\\d+";
const SELECTION_FORMAT: &str = "Expected selection format of \\d+,\\d(,\\d+)*";

impl From<IntRangeOption> for Vec<i64> {
    fn from(value: IntRangeOption) -> Self {
        match value {
            IntRangeOption::Single(i) => vec![i],
            IntRangeOption::Range(start, end) => (start..=end).collect(),
            IntRangeOption::Selection(v) => v,
        }
    }
}

impl FromStr for IntRangeOption {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if s.contains('-') {
            let mut ranges = s.split('-');
            let left_num = ranges
                .next()
                .context(RANGE_FORMAT)?
                .parse::<i64>()
                .context(RANGE_FORMAT)?;

            let right_num = ranges
                .next()
                .context(RANGE_FORMAT)?
                .parse::<i64>()
                .context(RANGE_FORMAT)?;

            return Ok(Self::Range(left_num, right_num));
        }

        if s.contains(',') {
            let nums = s
                .split(',')
                .map(|s| s.parse::<i64>())
                .collect::<Result<_, _>>()
                .context(SELECTION_FORMAT)?;

            return Ok(Self::Selection(nums));
        }

        let num = s.parse::<i64>().context("Expected i64")?;
        Ok(Self::Single(num))
    }
}

static REQWEST_CLIENT: OnceCell<Client> = OnceCell::const_new();

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let session_token = args.session_token;
    let max_retries = args.max_retries;
    let delay = args.delay;
    let output_dir = args.output_dir;

    if tokio::fs::try_exists(&output_dir).await.unwrap_or(false) {
        println!("{:?} already exits", output_dir);
    } else {
        tokio::fs::create_dir_all(&output_dir).await?;
        println!("Created {:?}", output_dir);
    }

    println!("Saving inputs to: {:?}", output_dir);

    REQWEST_CLIENT
        .get_or_init(|| async move {
            let headers = HeaderMap::from_iter(vec![(
                HeaderName::from_static("cookie"),
                HeaderValue::from_str(&format!("session={}", session_token)).unwrap(),
            )]);

            Client::builder().default_headers(headers).build().unwrap()
        })
        .await;

    println!("Scraping inputs for {:?}", args.days.clone());
    let days: Vec<_> = args.days.into();

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

    js.join_all().await.into_iter().for_each(|res| {
        if let Err(e) = res {
            eprintln!("Error scraping {}", e);
        };
    });
    Ok(())
}

async fn scrape_day(
    year: u32,
    day: i64,
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
            return Err(anyhow!("No question found for day {day}"));
        };

        if retries >= max_retries {
            return Err(anyhow!(e));
        }
        retries += 1;
        tokio::time::sleep(Duration::from_secs(delay as u64)).await;
    };

    let path = output_dir.join(format!("{:0>2}.txt", day));
    tokio::fs::write(path, body.as_bytes()).await?;

    println!("Saved input for Advent of Code {year}/{day:0>2}");

    Ok(())
}
