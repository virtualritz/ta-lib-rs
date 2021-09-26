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

define_values_period_fn!(
    /// Compute [Simple Moving Average](https://www.tadoc.org/indicator/SMA.htm) over a period (in days).
    ///
    /// Returns a tuple containing the list of SMA values and the
    /// index of the first close to have an associated SMA value.
    =>
    simple_moving_average,
    TA_SMA
);

pub enum MovingAverageType {
    SimpleMovingAverage = ta::TA_MAType_TA_MAType_SMA as _,
    ExponentialMovingAverage = ta::TA_MAType_TA_MAType_EMA as _,
    WeightedMovingAverage = ta::TA_MAType_TA_MAType_WMA as _,
    DoubleExponentialMovingAverage = ta::TA_MAType_TA_MAType_DEMA as _,
    TripleExponentialMovingAverage = ta::TA_MAType_TA_MAType_TEMA as _,
    TriangularMovingAverage = ta::TA_MAType_TA_MAType_TRIMA as _,
    KaufmanAdaptiveMovingAverage = ta::TA_MAType_TA_MAType_KAMA as _,
    MESAAdaptiveMovingAverage = ta::TA_MAType_TA_MAType_MAMA as _,
    TripleGeneralizedDoubleExponentialMovingAverage = ta::TA_MAType_TA_MAType_T3 as _,
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
    moving_average_type: Option<MovingAverageType>
) -> Result<(Vec<f64>, Vec<f64>, Vec<f64>, usize), Error> {
    assert!(!input.is_empty());

    let mut out_begin = MaybeUninit::<i32>::uninit();
    let mut out_size = MaybeUninit::<i32>::uninit();
    let mut out_upper_band: Vec<f64> = Vec::with_capacity(input.len());
    let mut out_middle_band: Vec<f64> = Vec::with_capacity(input.len());
    let mut out_lower_band: Vec<f64> = Vec::with_capacity(input.len());

    unsafe {
        let ret_code = ta::TA_BBANDS(
            0,
            (input.len() - 1) as _,
            input.as_ptr(),
            if let Some(period) = period { period as _ } else { ta::TA_INTEGER_DEFAULT },
            num_std_deviations_up.unwrap_or(ta::TA_REAL_DEFAULT),
            num_std_deviations_down.unwrap_or(ta::TA_REAL_DEFAULT),
            moving_average_type.unwrap_or(MovingAverageType::ExponentialMovingAverage) as _,
            out_begin.as_mut_ptr(),
            out_size.as_mut_ptr(),
            out_upper_band.as_mut_ptr(),
            out_middle_band.as_mut_ptr(),
            out_lower_band.as_mut_ptr(),
        );

        match ret_code {
            ta::TA_RetCode_TA_SUCCESS => {
                out_upper_band.set_len(out_size.assume_init() as _);
                out_middle_band.set_len(out_size.assume_init() as _);
                out_lower_band.set_len(out_size.assume_init() as _);

                Ok((out_upper_band, out_middle_band, out_lower_band, out_begin.assume_init() as _,))
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
            ta::TA_RetCode_TA_SUCCESS => {
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
