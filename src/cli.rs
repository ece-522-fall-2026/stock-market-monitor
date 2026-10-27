use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "stock-monitor",
    version,
    about = "Six months of closing prices; chart saved under tmp/",
    long_about = "Pass a ticker, or run with no args and type symbols interactively.\n\
                  Chart: tmp/<TICKER>_chart.png. Empty line at the prompt to quit."
)]
pub struct Cli {
    #[arg(value_parser = parse_ticker)]
    pub ticker: Option<String>,
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
