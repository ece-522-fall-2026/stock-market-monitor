# stock-market-monitor

## Crates

| Crate | Why |
|---|---|
| `clap` | Command-line parsing and `--help` |
| `chrono` | Dates for quotes and the chart's x-axis |
| `yahoo_finance_api` (planned) | Download daily stock quotes |
| `tokio` (planned) | Async runtime required by `yahoo_finance_api` |
| `plotters` (planned) | Draw the chart and save it as a PNG |
## Financial Analysis Algorithm

The stock analysis is implemented in `src/analysis.rs` using the `summarize()` function. It processes the daily stock records retrieved over the last six months.

### Minimum and Maximum Closing Prices

The algorithm iterates through all trading days and compares their closing prices to identify the lowest and highest closing prices and their corresponding dates. These records are stored in `Summary.min` and `Summary.max`.

### Volatility Calculation

The intraday volatility percentage is calculated using:

**Volatility (%) = ((High - Low) / Close) × 100**

Where:
- `High` is the highest price during the trading day.
- `Low` is the lowest price during the trading day.
- `Close` is the closing price.

A day is classified as volatile only when:

`((High - Low) / Close) × 100 > 2`

Days with exactly 2% variation are not considered volatile. Only records with a positive closing price are evaluated for volatility.

Volatile days are stored in `Summary.volatile` and passed to the plotting module to generate blue high-low-close error bars.

### Algorithm Efficiency

The `summarize()` function scans the records once, so its time complexity is **O(n)**, where `n` is the number of trading days. It uses **O(v)** additional space to store `v` volatile days.

### Unit Testing

Three unit tests verify:

1. Identification of the minimum and maximum closing prices.
2. Selection of days with volatility strictly greater than 2%, excluding exactly 2%.
3. Returning `None` when the input is empty.

All three analysis unit tests passed in the Task 3 development branch.
