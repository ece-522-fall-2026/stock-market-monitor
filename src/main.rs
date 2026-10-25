use std::io::{self, Write};
use std::path::Path;
use std::process::ExitCode;

use clap::Parser;
use stock_monitor::{parse_ticker, Cli};

fn main() -> ExitCode {
    let cli = Cli::parse();

    if let Some(ticker) = cli.ticker.as_deref() {
        try_run(ticker, cli.output.as_deref());
    }

    loop {
        print!("Enter stock symbol (empty to quit): ");
        if io::stdout().flush().is_err() {
            break;
        }

        let mut line = String::new();
        if io::stdin().read_line(&mut line).is_err() {
            eprintln!("error: could not read input");
            break;
        }

        let input = line.trim();
        if input.is_empty() {
            break;
        }

        let ticker = match parse_ticker(input) {
            Ok(ticker) => ticker,
            Err(message) => {
                eprintln!("error: invalid ticker: {message}");
                continue;
            }
        };

        try_run(&ticker, None);
    }

    ExitCode::SUCCESS
}

fn try_run(ticker: &str, output: Option<&Path>) {
    if let Err(error) = stock_monitor::run_for_ticker(ticker, output) {
        eprintln!("error: {error}");
    }
}
