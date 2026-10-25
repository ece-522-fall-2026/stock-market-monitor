pub mod analysis;
pub mod cli;
pub mod error;
pub mod fetch;
pub mod model;
pub mod plot;

use std::path::Path;
use std::process::Command;

pub use cli::Cli;
pub use error::AppError;

pub fn run(cli: &Cli) -> Result<(), AppError> {
    let days = fetch::fetch_quotes(&cli.ticker)?;
    let summary = analysis::summarize(&days).ok_or_else(|| AppError::NoData(cli.ticker.clone()))?;

    // Print minimum closing price and its date
    println!(
        "Minimum closing price: {:.2} on {}",
        summary.min.close, summary.min.date
    );

    // Print maximum closing price and its date
    println!(
        "Maximum closing price: {:.2} on {}",
        summary.max.close, summary.max.date
    );

    let output = cli.output_path();
    plot::draw_chart(&cli.ticker, &days, &summary, &output)?;
    open_file(&output);
    Ok(())
}

fn open_file(path: &Path) {
    let opener = if cfg!(target_os = "macos") {
        "open"
    } else if cfg!(target_os = "windows") {
        "explorer"
    } else {
        "xdg-open"
    };
    if let Err(e) = Command::new(opener).arg(path).spawn() {
        eprintln!("warning: could not open {}: {e}", path.display());
    }
}
