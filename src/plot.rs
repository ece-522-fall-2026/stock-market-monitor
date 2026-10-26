use std::collections::HashSet;
use std::path::Path;

use plotters::prelude::*;

use crate::error::AppError;
use crate::model::{Day, Summary};

fn plot_err(error: impl ToString) -> AppError {
    AppError::Plot(error.to_string())
}

fn price_range(days: &[Day]) -> (f64, f64) {
    let min_price = days.iter().map(|day| day.low).fold(f64::INFINITY, f64::min);
    let max_price = days.iter().map(|day| day.high).fold(f64::NEG_INFINITY, f64::max);
    (min_price, max_price)
}

pub fn draw_chart(
    ticker: &str,
    days: &[Day],
    summary: &Summary,
    out: &Path,
) -> Result<(), AppError> {
    if days.is_empty() {
        return Err(AppError::Plot("No data to plot".to_string()));
    }

    let (mut min_price, mut max_price) = price_range(days);
    let padding = (max_price - min_price) * 0.05;

    if min_price == max_price {
        min_price -= 1.0;
        max_price += 1.0;
    }

    let root = BitMapBackend::new(out, (1000, 600)).into_drawing_area();
    root.fill(&WHITE).map_err(plot_err)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(format!("{} closing price", ticker), ("sans-serif", 30))
        .margin(15)
        .x_label_area_size(60)
        .y_label_area_size(60)
        .build_cartesian_2d(
            0..days.len() as i32,
            (min_price - padding)..(max_price + padding),
        )
        .map_err(plot_err)?;

    chart
        .configure_mesh()
        .x_desc("Date")
        .y_desc("Price")
        .x_labels(16)
        .x_label_formatter(&|x| {
            days.get(*x as usize)
                .map(|day| day.date.to_string())
                .unwrap_or_default()
        })
        .draw()
        .map_err(plot_err)?;

    let close_line = days
        .iter()
        .enumerate()
        .map(|(index, day)| (index as i32, day.close));
    chart.draw_series(LineSeries::new(close_line, &RED)).map_err(plot_err)?;

    let volatile_dates: HashSet<_> = summary.volatile.iter().map(|day| day.date).collect();
    let error_bars = days.iter().enumerate().filter_map(|(index, day)| {
        volatile_dates.contains(&day.date).then(|| {
            ErrorBar::new_vertical(index as i32, day.low, day.close, day.high, BLUE, 8)
        })
    });
    chart.draw_series(error_bars).map_err(plot_err)?;

    root.present().map_err(plot_err)?;
    Ok(())
}
