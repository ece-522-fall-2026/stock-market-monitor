use crate::model::{Day, Summary};

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
