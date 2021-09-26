# `ta-lib`

High level Rust wrapper around the Technical Analysis library.
## Caveat

This crate only exposes the functions I need myself from the original TA lib.
Which has around 200 of them. Feel free to expose more and open a PR. Wrapping
a missing indicator is usually a one line addtion with the existing wrapper
generation macros in the crate.

The following indicators are currently wrapped:

* ADX – Average Directional (Movement) Index.
* ATR – Average True Range.
* BBANDS – Bollinger Bands.
* -DI – Minus Directional Indicator.
* +DI – Plus Directional Indicator.
* EMA – Exponential Moving Average.
* NATR – Normalized Average True Range.
* OBV – On Balance Volume.
* SMA – Simple Moving Average.
* TRANGE – True Range.

## Alternatives

I sugget using the [`ta`](https://crates.io/crates/ta/) crate instead, which is
a rewrite in Rust of *some* parts of TA but with better ergonomics than this
crate here. But it also only implements parts of the original TA lib indicators.

I.e. if you need to use an indicator missing from `ta` this crate may get you
there quickly as a workaround.
