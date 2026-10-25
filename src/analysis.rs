use crate::model::{Day, Summary};

fn is_volatile(day: &Day) -> bool {
    day.close > 0.0 && (day.high - day.low) / day.close * 100.0 > 2.0
}

pub fn summarize(days: &[Day]) -> Option<Summary> {
    let by_close = |a: &Day, b: &Day| a.close.total_cmp(&b.close);

    Some(Summary {
        min: days.iter().copied().min_by(by_close)?,
        max: days.iter().copied().max_by(by_close)?,
        volatile: days.iter().copied().filter(is_volatile).collect(),
    })
}
