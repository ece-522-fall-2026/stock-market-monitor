use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "stock-monitor",
    version,
    about,
    long_about = "Charts the last six months of daily closing prices for a stock.\n\n\
                  Days where the intraday range (high - low) exceeds 2% of the close are\n\
                  marked as volatile. The minimum and maximum closing prices and their\n\
                  dates are printed, and the chart is saved as a PNG image.\n\n\
                  After each chart, you can enter another ticker; press Enter on an empty\n\
                  line to quit."
)]
pub struct Cli {
    #[arg(value_parser = parse_ticker)]
    pub ticker: Option<String>,

    #[arg(short, long, value_name = "FILE")]
    pub output: Option<PathBuf>,
}

pub fn parse_ticker(s: &str) -> Result<String, String> {
    let valid = !s.is_empty()
        && s.len() <= 10
        && s.chars()
            .all(|c| c.is_ascii_alphanumeric() || ".-^".contains(c));

    if valid {
        Ok(s.to_ascii_uppercase())
    } else {
        Err("must be 1-10 characters: letters, digits, '.', '-' or '^'".into())
    }
}
