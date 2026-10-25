use crate::model::{Day, Summary};

// find the real min/max closes and the volatile days (range > 2% of close).
pub fn summarize(days: &[Day]) -> Option<Summary> {
    let first = *days.first()?;

    let mut min = first;
    let mut max = first;
    let mut volatile = Vec::new();

    for day in days {
        if day.close < min.close {
            min = *day;
        }
        if day.close > max.close {
            max = *day;
        }
        if day.close > 0.0 && (day.high - day.low) / day.close * 100.0 > 2.0 {
            volatile.push(*day);
        }
    }

    Some(Summary { min, max, volatile })
}
#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn day(date: (i32, u32, u32), high: f64, low: f64, close: f64) -> Day {
        Day {
            date: NaiveDate::from_ymd_opt(date.0, date.1, date.2).unwrap(),
            open: close,
            high,
            low,
            close,
        }
    }

    #[test]
    fn finds_min_and_max_closing_prices() {
        let days = [
            day((2026, 4, 20), 205.0, 195.0, 200.0),
            day((2026, 4, 21), 195.0, 185.0, 190.0),
            day((2026, 4, 22), 215.0, 205.0, 210.0),
        ];

        let summary = summarize(&days).unwrap();

        assert_eq!(summary.min.close, 190.0);
        assert_eq!(summary.min.date, days[1].date);
        assert_eq!(summary.max.close, 210.0);
        assert_eq!(summary.max.date, days[2].date);
    }

    #[test]
    fn selects_only_days_above_two_percent() {
        let days = [
            day((2026, 4, 20), 102.0, 100.0, 100.0), // Exactly 2%
            day((2026, 4, 21), 103.0, 100.0, 100.0), // 3%
            day((2026, 4, 22), 101.0, 100.0, 100.0), // 1%
        ];

        let summary = summarize(&days).unwrap();

        assert_eq!(summary.volatile.len(), 1);
        assert_eq!(summary.volatile[0].date, days[1].date);
    }

    #[test]
    fn empty_data_returns_none() {
        assert!(summarize(&[]).is_none());
    }
}
