# `ta-lib-sys`
[![Build Status](https://travis-ci.org/virtualritz/ta-lib-rs.svg?branch=master)](https://travis-ci.org/virtualritz/ta-lib-rs)

The Technical Analysis library includes 200 financial indicators such as ADX,
MACD, RSI, Stochastic, Bollinger Bands, as well as Candlestick pattern
recognition.

This crate includes the C source code the TA-Lib v0.4.0. This is built from
source and Rust-wrapped by default. Alternatively, the system’s TA-Lib may be
used. See Cargo features below.

## Cargo Features
* `use_system_lib` – Use the system's installed C TA lib instead of building
  from source.
