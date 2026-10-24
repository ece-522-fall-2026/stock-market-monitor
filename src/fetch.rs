use chrono::{DateTime, Utc};
use yahoo_finance_api::YahooConnector;

use crate::error::AppError;
use crate::model::Day;


pub fn fetch_quotes(ticker: &str) -> Result<Vec<Day>, AppError> {
    let ticker = ticker.trim();

    if ticker.is_empty() {
        return Err(AppError::Fetch(
            "stock ticker cannot be empty".to_string(),
        ));
    }

    let provider = YahooConnector::new()
        .map_err(|e| AppError::Fetch(e.to_string()))?;

    let response = provider
        .get_quote_range(ticker, "1d", "6mo")
        .map_err(|e| AppError::Fetch(e.to_string()))?;

    let quotes = response
        .quotes()
        .map_err(|e| AppError::Fetch(e.to_string()))?;

    if quotes.is_empty() {
        return Err(AppError::NoData(ticker.to_string()));
    }

    let mut days = Vec::new();

    for quote in quotes {
        if !quote.open.is_finite()
            || !quote.high.is_finite()
            || !quote.low.is_finite()
            || !quote.close.is_finite()
            || quote.high < quote.low
            || quote.close <= 0.0
        {
            continue;
        }

        let date = DateTime::<Utc>::from_timestamp(quote.timestamp, 0)
            .ok_or_else(|| {
                AppError::Fetch("invalid quote timestamp".to_string())
            })?
            .date_naive();

        days.push(Day {
            date,
            open: quote.open,
            high: quote.high,
            low: quote.low,
            close: quote.close,
        });
    }

    if days.is_empty() {
        return Err(AppError::NoData(ticker.to_string()));
    }

    days.sort_by_key(|day| day.date);

    Ok(days)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fetch_real_stock_data() {
        let result = fetch_quotes("AAPL");

        match result {
            Ok(days) => {
                println!("Number of records: {}", days.len());

                for day in days.iter().take(5) {
                    println!(
                        "{} | Open: {:.2} | High: {:.2} | Low: {:.2} | Close: {:.2}",
                        day.date, day.open, day.high, day.low, day.close
                    );
                }

                assert!(!days.is_empty());
                assert!(days.len() > 5);

                assert!(
                    days.windows(2).all(|pair| pair[0].date <= pair[1].date),
                    "Dates should be sorted chronologically"
                );
            }
            Err(error) => panic!("Fetching AAPL data failed: {error}"),
        }
    }
}