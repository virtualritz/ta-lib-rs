//#![warn(missing_docs)]
//! High level, safe wrapper around the [Technical Analysis
//! library](https://ta-lib.org/).
//!
//! This library includes 200 financial indicators such as
//! [`ADX`](average_directional_movement_index), MACD, RSI, Stochastic,
//! Bollinger Bands, as well as Candlestick pattern recognition.
//!
//! ## EXample
//!
//! ```
//! use ta_lib::simple_moving_average;
//!
//! let close_prices = [
//!     1.087010, 1.087120, 1.087080, 1.087170, 1.087110, 1.087010, 1.087100, 1.087120, 1.087110,
//!     1.087080, 1.087000, 1.086630, 1.086630, 1.086610, 1.086630, 1.086640, 1.086650, 1.086650,
//!     1.086670, 1.086630,
//! ];
//!
//! let period = 10;
//!
//! let (sma_values, begin) = simple_moving_average(&close_prices, Some(period)).unwrap();
//! ```
//!
//! ## Cargo Features
//! * `use_system_lib` – Use the system's installed TA lib instead of building
//!   from source.
//!
//! By deafult the wrapped TA lib is built from source included with the
//! `ta-lib-sys` crate.

use concat_idents::concat_idents;
use std::mem::{transmute, MaybeUninit};
use ta_lib_sys as ta;

mod macros;
use macros::*;

#[derive(Debug, Clone)]
pub struct Error(String);

define_high_low_close_period_fn!(
    /// Compute [Average Directional (Movement) Index](https://www.tadoc.org/indicator/ADX.htm) over a period.
    ///
    /// Returns a tuple containing the list of ADX values and the
    /// index of the first candle to have an associated ADX value.
    =>
    average_directional_movement_index,
    ADX
);

define_high_low_close_period_fn!(
    /// Compute [Average True Range](https://www.tadoc.org/indicator/ATR.htm) over a period.
    ///
    /// Returns a tuple containing the list of ATR values and the
    /// index of the first candle to have an associated ATR value.
    =>
    average_true_range,
    ATR
);

define_high_low_close_period_fn!(
    /// Compute [Normalized Average True Range](https://www.tadoc.org/indicator/NATR.htm) over a period.
    ///
    /// Returns a tuple containing the list of NATR values and the
    /// index of the first candle to have an associated NATR value.
    =>
    normalized_average_true_range,
    NATR
);

define_high_low_close_period_fn!(
    /// Compute [Negative Directional Indicator](https://www.tadoc.org/indicator/MINUS_DI.htm) over a period.
    ///
    /// Returns a tuple containing the list of -DI values and the
    /// index of the first candle to have an associated -DI value.
    =>
    negative_directional_indicator,
    MINUS_DI
);

define_high_low_close_period_fn!(
    /// Compute [Positive Directional Indicator](https://www.tadoc.org/indicator/PLUS_DI.htm) over a period.
    ///
    /// Returns a tuple containing the list of +DI values and the
    /// index of the first candle to have an associated +DI value.
    =>
    positive_directional_indicator,
    PLUS_DI
);

define_high_low_close_fn!(
    /// Compute [True Range](https://www.tadoc.org/indicator/TRANGE.htm) over a period.
    ///
    /// Returns a tuple containing the list of TRANGE values and the
    /// index of the first candle to have an associated TRANGE value.
    =>
    true_range,
    TRANGE
);

define_values_period_fn!(
    /// Compute [Exponential Moving Average](https://www.tadoc.org/indicator/EMA.htm) over a period.
    ///
    /// Returns a tuple containing the list of EMA values and the
    /// index of the first candle to have an associated EMA value.
    =>
    exponential_moving_average,
    EMA
);

define_values_period_fn!(
    /// Compute [Simple Moving Average](https://www.tadoc.org/indicator/SMA.htm) over a period.
    ///
    /// Returns a tuple containing the list of SMA values and the
    /// index of the first close to have an associated SMA value.
    =>
    simple_moving_average,
    SMA
);

#[repr(C)]
pub enum MovingAverageType {
    SimpleMovingAverage = ta::MAType::MAType_SMA as _,
    ExponentialMovingAverage = ta::MAType::MAType_EMA as _,
    WeightedMovingAverage = ta::MAType::MAType_WMA as _,
    DoubleExponentialMovingAverage = ta::MAType::MAType_DEMA as _,
    TripleExponentialMovingAverage = ta::MAType::MAType_TEMA as _,
    TriangularMovingAverage = ta::MAType::MAType_TRIMA as _,
    KaufmanAdaptiveMovingAverage = ta::MAType::MAType_KAMA as _,
    MesaAdaptiveMovingAverage = ta::MAType::MAType_MAMA as _,
    TripleGeneralizedDoubleExponentialMovingAverage = ta::MAType::MAType_T3 as _,
}

/// Compute [Bollinger Bands](https://www.tadoc.org/indicator/BBANDS.htm).
///
/// Returns a tuple containing the upper, middle and lower BBANDS values and the
/// index of the first candle to have an associated BBANS value.
pub fn bollinger_bands(
    input: &[f64],
    period: Option<usize>,
    num_std_deviations_up: Option<f64>,
    num_std_deviations_down: Option<f64>,
    moving_average_type: Option<MovingAverageType>,
) -> Result<(Vec<f64>, Vec<f64>, Vec<f64>, usize), Error> {
    assert!(!input.is_empty());

    let mut out_begin = MaybeUninit::<i32>::uninit();
    let mut out_size = MaybeUninit::<i32>::uninit();
    let mut out_upper_band: Vec<f64> = Vec::with_capacity(input.len());
    let mut out_middle_band: Vec<f64> = Vec::with_capacity(input.len());
    let mut out_lower_band: Vec<f64> = Vec::with_capacity(input.len());

    unsafe {
        let ret_code = ta::BBANDS(
            0,
            (input.len() - 1) as _,
            input.as_ptr(),
            if let Some(period) = period {
                period as _
            } else {
                // ta::INTEGER_DEFAULT
                i32::MIN
            },
            num_std_deviations_up.unwrap_or(ta::REAL_DEFAULT),
            num_std_deviations_down.unwrap_or(ta::REAL_DEFAULT),
            transmute(moving_average_type.unwrap_or(MovingAverageType::ExponentialMovingAverage)),
            out_begin.as_mut_ptr(),
            out_size.as_mut_ptr(),
            out_upper_band.as_mut_ptr(),
            out_middle_band.as_mut_ptr(),
            out_lower_band.as_mut_ptr(),
        );

        match ret_code {
            ta::RetCode::SUCCESS => {
                out_upper_band.set_len(out_size.assume_init() as _);
                out_middle_band.set_len(out_size.assume_init() as _);
                out_lower_band.set_len(out_size.assume_init() as _);

                Ok((
                    out_upper_band,
                    out_middle_band,
                    out_lower_band,
                    out_begin.assume_init() as _,
                ))
            }
            _ => Err(Error(format!(
                "Could not compute OBV; error: {:?}",
                ret_code
            ))),
        }
    }
}

/*
#[test]
fn test_obv() {
    println!(
        "{:?}",
        on_balance_volume(&[1.0, 2.0, 3.0, 4.0], &[1.0, 2.0, 3.0, 4.0])
            .unwrap()
            .0
    );
}*/

/// Compute [On Balance Volume](https://www.tadoc.org/indicator/OBV.htm).
///
/// Returns a tuple containing the list of OBV values and the
/// index of the first candle to have an associated OBV value.
pub fn on_balance_volume(close: &[f64], volume: &[f64]) -> Result<(Vec<f64>, usize), Error> {
    assert!(!close.is_empty());
    assert!(close.len() <= volume.len());

    let mut out: Vec<f64> = Vec::with_capacity(close.len());
    let mut out_begin = MaybeUninit::<i32>::uninit();
    let mut out_size = MaybeUninit::<i32>::uninit();

    unsafe {
        let ret_code = ta::OBV(
            0,
            (close.len() - 1) as _,
            close.as_ptr(),
            volume.as_ptr(),
            out_begin.as_mut_ptr(),
            out_size.as_mut_ptr(),
            out.as_mut_ptr(),
        );

        match ret_code {
            ta::RetCode::SUCCESS => {
                out.set_len(out_size.assume_init() as _);
                Ok((out, out_begin.assume_init() as _))
            }
            _ => Err(Error(format!(
                "Could not compute OBV; error: {:?}",
                ret_code
            ))),
        }
    }
}

#[test]
fn test_on_balance_volume() {
    println!(
        "{:?}",
        on_balance_volume(&[1.0, 2.0, 3.0, 4.0], &[1.0, 2.0, 3.0, 4.0]).unwrap()
    );
}

#[test]
fn test_sma() {
    let close_prices = [
        1.087010, 1.087120, 1.087080, 1.087170, 1.087110, 1.087010, 1.087100, 1.087120, 1.087110,
        1.087080, 1.087000, 1.086630, 1.086630, 1.086610, 1.086630, 1.086640, 1.086650, 1.086650,
        1.086670, 1.086630,
    ];

    // Compute SMA, since we use a period of 10, the first 10 closes won't have
    // an sma value because there is not enough data, so begin will be set to
    // the index 20.
    let (sma_values, begin) = simple_moving_average(&close_prices, Some(10)).unwrap();

    // print values
    for (index, value) in sma_values.iter().enumerate() {
        println!("Close index {} = {}", begin + index + 1, value);
    }
}
