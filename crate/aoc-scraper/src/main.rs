use std::{path::PathBuf, str::FromStr, time::Duration};

use anyhow::{Context, Result, anyhow};
use clap::Parser;
use futures::{StreamExt, TryFutureExt, stream};
use reqwest::{
    Client,
    header::{HeaderMap, HeaderName, HeaderValue},
};
use scraper::{Html, Selector};
use tap::Pipe;
use tokio::sync::OnceCell;

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    #[arg(short, long, help = "The year/s to scrape")]
    pub year: IntRangeOption,
    #[arg(short, long, help = "The day/s to scrape")]
    pub day: IntRangeOption,
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

static PUZZLE_SELECTOR: OnceCell<Selector> = OnceCell::const_new();
async fn init_puzzle_selector() {
    const SELECTOR: &str = "body > main > article > pre:first-of-type";
    PUZZLE_SELECTOR
        .get_or_try_init(|| async { Selector::parse(SELECTOR) })
        .await
        .unwrap();
}

static OUTPUT_DIR: OnceCell<PathBuf> = OnceCell::const_new();
async fn init_output_dir(dir: PathBuf) {
    OUTPUT_DIR.get_or_init(|| async { dir }).await;
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let session_token = args.session_token;
    let max_retries = args.max_retries;
    let delay = args.delay;
    let output_dir = args.output_dir;

    tokio::join!(init_output_dir(output_dir), init_puzzle_selector());
    let output_dir = OUTPUT_DIR.get().unwrap();

    if tokio::fs::try_exists(output_dir).await.unwrap_or(false) {
        println!("{:?} already exits", output_dir);
    } else {
        tokio::fs::create_dir_all(output_dir).await?;
        println!("Created {:?}", output_dir);
    }

    println!("Saving inputs to: {:?}", output_dir);

    let reqwest_client = {
        let headers = HeaderMap::from_iter(vec![(
            HeaderName::from_static("cookie"),
            HeaderValue::from_str(&format!("session={}", session_token)).unwrap(),
        )]);

        Client::builder().default_headers(headers).build().unwrap()
    };

    println!(
        "Scraping inputs for year: {:?} and day: {:?}",
        args.year.clone(),
        args.day.clone()
    );
    let years: Vec<_> = args.year.into();
    let days: Vec<_> = args.day.into();

    let mut js = tokio::task::JoinSet::new();
    years.iter().copied().for_each(|year| {
        days.iter().copied().for_each(|day| {
            js.spawn(scrape_puzzle_input(
                reqwest_client.clone(),
                year,
                day,
                max_retries,
                delay,
            ));

            js.spawn(fetch_input(
                reqwest_client.clone(),
                year,
                day,
                max_retries,
                delay,
            ));
        });
    });

    js.join_all().await.into_iter().for_each(|res| {
        if let Err(e) = res {
            eprintln!("Error scraping {}", e);
        };
    });
    Ok(())
}

async fn scrape_puzzle_input(
    cli: Client,
    year: i64,
    day: i64,
    max_retries: u8,
    delay: u8,
) -> Result<()> {
    let url = format!("https://adventofcode.com/{}/day/{}", year, day);

    let fetch_day_desc = || async {
        cli.get(&url)
            .send()
            .and_then(|response| response.text())
            .await
    };

    let mut retries = 0;
    let body = loop {
        let res = fetch_day_desc().await;
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

    // Typically, the puzzle input is the first pre > code of each article
    let htmls: Vec<_> = Html::parse_document(&body)
        .select(PUZZLE_SELECTOR.get().unwrap())
        .map(|el| {
            el.child_elements()
                .next()
                .and_then(|el| el.text().next())
                .map(|s| s.to_string())
        })
        .collect::<Option<_>>()
        .unwrap();

    htmls
        .iter()
        .enumerate()
        .pipe(stream::iter)
        .then(|(i, html)| async move {
            let path =
                OUTPUT_DIR
                    .get()
                    .unwrap()
                    .join(format!("{}_{:0>2}.test_{}.txt", year, day, i + 1));
            tokio::fs::write(path, html.as_bytes()).await.unwrap();
            println!(
                "Saved puzzle input for Advent of Code {}/{:0>2}, part {}",
                year,
                day,
                i + 1
            );
        })
        .collect::<Vec<_>>()
        .await;

    Ok(())
}

async fn fetch_input(cli: Client, year: i64, day: i64, max_retries: u8, delay: u8) -> Result<()> {
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);

    let fetch_input = || async {
        cli.get(&url)
            .send()
            .and_then(|response| response.text())
            .await
    };

    let mut retries = 0;
    let input_text = loop {
        let res = fetch_input().await;
        let e = match res {
            Ok(text) => {
                if text.contains("404 Not Found") {
                    return Err(anyhow!("No question found for day {day}"));
                }
                break text;
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

    if input_text.contains("Please log in") {
        return Err(anyhow!(
            "Advent of Code session token not provided or expired"
        ));
    }

    let path = OUTPUT_DIR
        .get()
        .unwrap()
        .join(format!("{}_{:0>2}.txt", year, day));
    tokio::fs::write(path, input_text.as_bytes()).await?;

    println!("Saved input for Advent of Code {year}/{day:0>2}");

    Ok(())
}
