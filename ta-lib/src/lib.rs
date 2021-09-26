//#![warn(missing_docs)]
//! The Technical Analysis library includes 200 financial indicators such as
//! [`ADX`](average_directional_movement_index), MACD, RSI, Stochastic,
//! Bollinger Bands, as well as Candlestick pattern recognition.
//!
//! ## Cargo Features
//! * `use_system_lib` – Use the system's installed C TA lib instead of building
//!   from source.
//!
//!   By deafult the C TA lib is built from source included with the `ta-lib-sys`
//!   crate.
use concat_idents::concat_idents;
use std::mem::MaybeUninit;
use ta_lib_sys as ta;

#[macro_use]
mod macros;

#[derive(Debug, Clone)]
pub struct Error(String);

define_high_low_close_period_fn!(
    /// Compute [Average Directional (Movement) Index](https://www.tadoc.org/indicator/ADX.htm) over a period (in days).
    ///
    /// Returns a tuple containing the list of ADX values and the
    /// index of the first candle to have an associated ADX value.
    =>
    average_directional_movement_index,
    TA_ADX
);

define_high_low_close_period_fn!(
    /// Compute [Average True Range](https://www.tadoc.org/indicator/ATR.htm) over a period (in days).
    ///
    /// Returns a tuple containing the list of ATR values and the
    /// index of the first candle to have an associated ATR value.
    =>
    average_true_range,
    TA_ATR
);

define_high_low_close_period_fn!(
    /// Compute [Normalized Average True Range](https://www.tadoc.org/indicator/NATR.htm) over a period (in days).
    ///
    /// Returns a tuple containing the list of NATR values and the
    /// index of the first candle to have an associated NATR value.
    =>
    normalized_average_true_range,
    TA_NATR
);

define_high_low_close_period_fn!(
    /// Compute [Minus Directional Indicator](https://www.tadoc.org/indicator/MINUS_DI.htm) over a period (in days).
    ///
    /// Returns a tuple containing the list of -DI values and the
    /// index of the first candle to have an associated -DI value.
    =>
    minus_directional_indicator,
    TA_MINUS_DI
);

define_high_low_close_period_fn!(
    /// Compute [Plus Directional Indicator](https://www.tadoc.org/indicator/PLUS_DI.htm) over a period (in days).
    ///
    /// Returns a tuple containing the list of +DI values and the
    /// index of the first candle to have an associated +DI value.
    =>
    plus_directional_indicator,
    TA_PLUS_DI
);

define_high_low_close_fn!(
    /// Compute [True Range](https://www.tadoc.org/indicator/TRANGE.htm) over a period (in days).
    ///
    /// Returns a tuple containing the list of TRANGE values and the
    /// index of the first candle to have an associated TRANGE value.
    =>
    true_range,
    TA_TRANGE
);

define_values_period_fn!(
    /// Compute [Exponential Moving Average](https://www.tadoc.org/indicator/EMA.htm) over a period (in days).
    ///
    /// Returns a tuple containing the list of EMA values and the
    /// index of the first candle to have an associated EMA value.
    =>
    exponential_moving_average,
    TA_EMA
);

/// Compute [On Balance Volume](https://www.tadoc.org/indicator/OBV.htm).
///
/// Returns a tuple containing the list of OBV values and the
/// index of the first candle to have an associated OBV value.
pub fn on_balance_volume(real: &[f64], volume: &[f64]) -> Result<(Vec<f64>, usize), Error> {
    assert!(!real.is_empty());
    assert!(real.len() <= volume.len());

    let mut out: Vec<f64> = Vec::with_capacity(real.len());
    let mut out_begin = MaybeUninit::<i32>::uninit();
    let mut out_size = MaybeUninit::<i32>::uninit();

    unsafe {
        let ret_code = ta::TA_OBV(
            0,
            (real.len() - 1) as _,
            real.as_ptr(),
            volume.as_ptr(),
            out_begin.as_mut_ptr(),
            out_size.as_mut_ptr(),
            out.as_mut_ptr(),
        );

        match ret_code {
            ta::TA_RetCode::TA_SUCCESS => {
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
fn test_obv() {
    println!(
        "{:?}",
        on_balance_volume(&[1.0, 2.0, 3.0, 4.0], &[1.0, 2.0, 3.0, 4.0])
            .unwrap()
            .0
    );
}

/// Compute [Simple Moving Average](https://www.tadoc.org/indicator/SMA.htm) over a period (in days).
///
/// Returns a tuple containing the list of SMA values and the index of the first
/// close to have an associated SMA value.
pub fn simple_moving_average(
    close_prices: &[f64],
    period: Option<usize>,
) -> Result<(Vec<f64>, usize), Error> {
    assert!(!close_prices.is_empty());

    let mut out: Vec<f64> = Vec::with_capacity(close_prices.len());
    let mut out_begin = MaybeUninit::<i32>::uninit();
    let mut out_size = MaybeUninit::<i32>::uninit();

    unsafe {
        let ret_code = ta::TA_MA(
            0,
            (close_prices.len() - 1) as _,
            close_prices.as_ptr(),
            if let Some(period) = period {
                period as _
            } else {
                ta::TA_INTEGER_DEFAULT
            },
            ta::TA_MAType::TA_MAType_SMA,
            out_begin.as_mut_ptr(),
            out_size.as_mut_ptr(),
            out.as_mut_ptr(),
        );

        match ret_code {
            ta::TA_RetCode::TA_SUCCESS => {
                out.set_len(out_size.assume_init() as _);
                Ok((out, out_begin.assume_init() as _))
            }
            _ => Err(Error(format!(
                "Could not compute SMA; error: {:?}",
                ret_code
            ))),
        }
    }
}

#[test]
fn test_sma() {
    let close_prices = [
        1.087010, 1.087120, 1.087080, 1.087170, 1.087110, 1.087010, 1.087100, 1.087120, 1.087110,
        1.087080, 1.087000, 1.086630, 1.086630, 1.086610, 1.086630, 1.086640, 1.086650, 1.086650,
        1.086670, 1.086630,
    ];

    // compute sma, since we use a period of 10, the first 10 closes won't have
    // an sma value because there is not enough data, so begin will be set to
    // the index 29
    let (sma_values, begin) = simple_moving_average(&close_prices, Some(10)).unwrap();

    // print values
    for (index, value) in sma_values.iter().enumerate() {
        println!("Close index {} = {}", begin + index + 1, value);
    }
}
