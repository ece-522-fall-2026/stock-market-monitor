use std::fmt;

#[derive(Debug)]
pub enum AppError {
    Fetch(String),
    NoData(String),
    TickerNotFound(String),
    Plot(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Fetch(reason) => write!(f, "failed to fetch quotes: {reason}"),
            Self::NoData(ticker) => write!(f, "no quote data available for {ticker}"),
            Self::TickerNotFound(ticker) => {
                write!(f, "ticker symbol not found: {ticker}")
            }
            Self::Plot(reason) => write!(f, "failed to draw chart: {reason}"),
        }
    }
}

impl std::error::Error for AppError {}
