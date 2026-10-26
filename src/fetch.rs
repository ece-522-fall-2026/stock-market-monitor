use chrono::{DateTime, Utc};
use yahoo_finance_api::{YahooConnector, YahooError};

use crate::error::AppError;
use crate::model::Day;

fn map_yahoo_error(e: YahooError, ticker: &str) -> AppError {
    match e {
        YahooError::FetchFailed(msg) if msg.contains("404") => {
            AppError::TickerNotFound(ticker.to_string())
        }
        YahooError::NoResult | YahooError::NoQuotes => AppError::NoData(ticker.to_string()),
        other => AppError::Fetch(other.to_string()),
    }
}

pub fn fetch_quotes(ticker: &str) -> Result<Vec<Day>, AppError> {
    let ticker = ticker.trim();

    if ticker.is_empty() {
        return Err(AppError::Fetch(
            "stock ticker cannot be empty".to_string(),
        ));
    }

    let provider = YahooConnector::new().map_err(|e| map_yahoo_error(e, ticker))?;

    let response = provider
        .get_quote_range(ticker, "1d", "6mo")
        .map_err(|e| map_yahoo_error(e, ticker))?;

    let quotes = response
        .quotes()
        .map_err(|e| map_yahoo_error(e, ticker))?;

    if quotes.is_empty() {
        return Err(AppError::NoData(ticker.to_string()));
    }

    let mut days: Vec<Day> = quotes
        .iter()
        .filter(|quote| {
            quote.open.is_finite()
                && quote.high.is_finite()
                && quote.low.is_finite()
                && quote.close.is_finite()
                && quote.high >= quote.low
                && quote.close > 0.0
        })
        .map(|quote| {
            let date = DateTime::<Utc>::from_timestamp(quote.timestamp, 0)
                .ok_or_else(|| AppError::Fetch("invalid quote timestamp".to_string()))?
                .date_naive();

            Ok(Day {
                date,
                open: quote.open,
                high: quote.high,
                low: quote.low,
                close: quote.close,
            })
        })
        .collect::<Result<Vec<Day>, AppError>>()?;

    if days.is_empty() {
        return Err(AppError::NoData(ticker.to_string()));
    }

    days.sort_by_key(|day| day.date);

    Ok(days)
}
