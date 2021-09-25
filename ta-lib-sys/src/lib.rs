#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
//! The Technical Analysis library includes 200 financial indicators such as
//! [`ADX`](average_directional_movement_index), MACD, RSI, Stochastic,
//! Bollinger Bands, as well as Candlestick pattern recognition.
//!
//! ## Cargo Features
//! * `use_system_lib` – Use the system's installed C TA lib instead of building
//!   from source.

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
