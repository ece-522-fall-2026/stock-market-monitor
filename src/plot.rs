use std::path::Path;

use plotters::prelude::*;

use crate::error::AppError;
use crate::model::{Day, Summary};

// TODO(part 4): draw the chart with plotters and save it to `out`.
pub fn draw_chart(
    _ticker: &str,
    _days: &[Day],
    _summary: &Summary,
    out: &Path,
) -> Result<(), AppError> {
    if _days.len() == 0 {
        return Err(AppError::Plot("No data to plot".to_string()));
    }

    let mut min_price = _days[0].low;
    let mut max_price = _days[0].high;

    for day in _days {
        if day.low < min_price {
            min_price = day.low;
        }
        if day.high > max_price {
            max_price = day.high;
        }
    }

    let padding = (max_price - min_price) * 0.05;

    if min_price == max_price {
        min_price -= 1.0;
        max_price += 1.0;
    }

    let root = BitMapBackend::new(out, (1000, 600)).into_drawing_area();

    root.fill(&WHITE)
        .map_err(|e| AppError::Plot(e.to_string()))?;

    let mut chart = ChartBuilder::on(&root)
        .caption(format!("{} closing price", _ticker), ("sans-serif", 30))
        .margin(15)
        .x_label_area_size(60)
        .y_label_area_size(60)
        .build_cartesian_2d(
            0.._days.len() as i32,
            (min_price - padding)..(max_price + padding),
        )
        .map_err(|e| AppError::Plot(e.to_string()))?;

    chart
        .configure_mesh()
        .x_desc("Date")
        .y_desc("Price")
        .x_labels(6)
        .x_label_formatter(&|x| {
            if *x >= 0 && (*x as usize) < _days.len() {
                _days[*x as usize].date.to_string()
            } else {
                String::new()
            }
        })
        .draw()
        .map_err(|e| AppError::Plot(e.to_string()))?;

    let mut points = Vec::new();

    for i in 0.._days.len() {
        let day = &_days[i];
        points.push((i as i32, day.close));
    }

    chart
        .draw_series(LineSeries::new(points, &RED))
        .map_err(|e| AppError::Plot(e.to_string()))?;

    for i in 0.._days.len() {
        let day = &_days[i];
        let mut volatile = false;

        for v in &_summary.volatile {
            if v.date == day.date {
                volatile = true;
            }
        }

        if volatile == true {
            chart
                .draw_series([ErrorBar::new_vertical(
                    i as i32,
                    day.low,
                    day.close,
                    day.high,
                    BLUE,
                    8,
                )])
                .map_err(|e| AppError::Plot(e.to_string()))?;
        }
    }

    root.present()
        .map_err(|e| AppError::Plot(e.to_string()))?;

    Ok(())
}
