pub mod analysis;
pub mod cli;
pub mod error;
pub mod fetch;
pub mod model;
pub mod plot;

use std::path::{Path, PathBuf};
use std::process::Command;

pub use cli::{parse_ticker, Cli};
pub use error::AppError;

pub fn run_for_ticker(ticker: &str) -> Result<(), AppError> {
    let days = fetch::fetch_quotes(ticker)?;
    let summary = analysis::summarize(&days).ok_or_else(|| AppError::NoData(ticker.to_string()))?;

    let output_path = PathBuf::from("tmp").join(format!("{}_chart.png", ticker));
    std::fs::create_dir_all("tmp")
        .map_err(|e| AppError::Plot(format!("could not create tmp: {e}")))?;
    plot::draw_chart(ticker, &days, &summary, &output_path)?;
    open_file(&output_path);
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
