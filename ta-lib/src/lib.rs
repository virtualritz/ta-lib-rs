use std::mem::MaybeUninit;
use ta_lib_sys::{TA_RetCode, TA_ATR, TA_OBV};

#[derive(Debug, Clone)]
pub struct Error(String);

/// Compute [ATR](http://www.tadoc.org/indicator/ATR.htm) over a period.
///
/// This function returns a tuple containing the list of ATR values and the
/// index of the first candle to have an associated ATR value.
pub fn atr(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    period: usize,
) -> Result<(Vec<f64>, usize), Error> {
    assert!(!close.is_empty());
    assert!(close.len() <= high.len());
    assert!(close.len() <= low.len());

    let mut out: Vec<f64> = Vec::with_capacity(close.len());
    let mut out_begin = MaybeUninit::<i32>::uninit();
    let mut out_size = MaybeUninit::<i32>::uninit();

    unsafe {
        let ret_code = TA_ATR(
            0,                      // index of the first close to use
            (close.len() - 1) as _, // index of the last close to use
            high.as_ptr(),          // pointer to the first element of the high vector
            low.as_ptr(),           // pointer to the first element of the low vector
            close.as_ptr(),         // pointer to the first element of the close vector
            period as _,            // period of the atr
            out_begin.as_mut_ptr(), // set to index of the first close to have an atr value
            out_size.as_mut_ptr(),  // set to number of atr values computed
            out.as_mut_ptr(),       // pointer to the first element of the output vector
        );

        match ret_code {
            TA_RetCode::TA_SUCCESS => {
                out.set_len(out_size.assume_init() as _);
                Ok((out, out_begin.assume_init() as _))
            }
            _ => Err(Error(format!("Could not compute ATR; error: {:?}", ret_code))),
        }
    }
}

/// Compute [OBV](http://www.tadoc.org/indicator/OBV.htm).
///
/// This function returns a tuple containing the list of OBV values and the
/// index of the first candle to have an associated OBV value.
pub fn obv(real: &[f64], volume: &[f64]) -> Result<(Vec<f64>, usize), Error> {
    assert!(!real.is_empty());
    assert!(real.len() <= volume.len());

    let mut out: Vec<f64> = Vec::with_capacity(real.len());
    let mut out_begin = MaybeUninit::<i32>::uninit();
    let mut out_size = MaybeUninit::<i32>::uninit();

    unsafe {
        let ret_code = TA_OBV(
            0,
            (real.len() - 1) as _,
            real.as_ptr(),
            volume.as_ptr(),
            out_begin.as_mut_ptr(),
            out_size.as_mut_ptr(),
            out.as_mut_ptr(),
        );

        match ret_code {
            TA_RetCode::TA_SUCCESS => {
                out.set_len(out_size.assume_init() as _);
                Ok((out, out_size.assume_init() as _))
            }
            _ => Err(Error(format!("Could not compute OBV; error: {:?}", ret_code))),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        println!("{:?}", super::atr(&[1.0, 2.0, 3.0, 4.0], &[1.0, 2.0, 3.0, 4.0], &[1.0, 2.0, 3.0, 4.0], 2).unwrap().0);
        println!("{:?}", super::obv(&[1.0, 2.0, 3.0, 4.0], &[1.0, 2.0, 3.0, 4.0]).unwrap().0);
    }
}
