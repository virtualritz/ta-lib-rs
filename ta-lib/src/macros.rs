macro_rules! define_high_low_close_period_fn {
    ($(#[$attr:meta])* => $fn_name:ident, $ta_fn_name:ident) => {
        $(#[$attr])*
        pub fn $fn_name(
            high: &[f64],
            low: &[f64],
            close: &[f64],
            period: Option<usize>,
        ) -> Result<(Vec<f64>, usize), Error> {
            assert!(!close.is_empty());
            assert!(close.len() <= high.len());
            assert!(close.len() <= low.len());

            let mut out: Vec<f64> = Vec::with_capacity(close.len());
            let mut out_begin = MaybeUninit::<i32>::uninit();
            let mut out_size = MaybeUninit::<i32>::uninit();

            unsafe {
                let ret_code = ta::$ta_fn_name(
                    0,
                    (close.len() - 1) as _,
                    high.as_ptr(),
                    low.as_ptr(),
                    close.as_ptr(),
                    if let Some(period) = period {
                        period as _
                    } else {
                        ta::TA_INTEGER_DEFAULT
                    },
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
                        "Could not compute function; error: {:?}",
                        ret_code
                    ))),
                }
            }
        }

        concat_idents!(test_name = test, $fn_name {
            #[test]
            fn test_name() {
                let high = [
                    1.087130, 1.087120, 1.087220, 1.087230, 1.087180, 1.087160, 1.087210, 1.087150, 1.087200,
                    1.087230, 1.087070, 1.087000, 1.086630, 1.086650, 1.086680, 1.086690, 1.086690, 1.086690,
                    1.086690, 1.086650,
                ];
                let low = [
                    1.087010, 1.087120, 1.087080, 1.087170, 1.087110, 1.087010, 1.087100, 1.087120, 1.087110,
                    1.087080, 1.087000, 1.086630, 1.086630, 1.086610, 1.086630, 1.086640, 1.086650, 1.086650,
                    1.086670, 1.086630,
                ];
                let close = [
                    1.087130, 1.087120, 1.087220, 1.087230, 1.087110, 1.087120, 1.087100, 1.087120, 1.087130,
                    1.087080, 1.087000, 1.086630, 1.086630, 1.086650, 1.086640, 1.086690, 1.086650, 1.086690,
                    1.086670, 1.086640,
                ];

                let (atr_values, begin) = $fn_name(&high, &low, &close, Some(7)).unwrap();

                // print values
                for (index, value) in atr_values.iter().enumerate() {
                    println!("index {} = {}", begin + index + 1, value);
                }
            }
        });
    };
}

macro_rules! define_high_low_close_fn {
    ($(#[$attr:meta])* => $fn_name:ident, $ta_fn_name:ident) => {
        $(#[$attr])*
        pub fn $fn_name(
            high: &[f64],
            low: &[f64],
            close: &[f64],
        ) -> Result<(Vec<f64>, usize), Error> {
            assert!(!close.is_empty());
            assert!(close.len() <= high.len());
            assert!(close.len() <= low.len());

            let mut out: Vec<f64> = Vec::with_capacity(close.len());
            let mut out_begin = MaybeUninit::<i32>::uninit();
            let mut out_size = MaybeUninit::<i32>::uninit();

            unsafe {
                let ret_code = ta::$ta_fn_name(
                    0,
                    (close.len() - 1) as _,
                    high.as_ptr(),
                    low.as_ptr(),
                    close.as_ptr(),
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
                        "Could not compute function; error: {:?}",
                        ret_code
                    ))),
                }
            }
        }

        concat_idents!(test_name = test, $fn_name {
            #[test]
            fn test_name() {
                let high = [
                    1.087130, 1.087120, 1.087220, 1.087230, 1.087180, 1.087160, 1.087210, 1.087150, 1.087200,
                    1.087230, 1.087070, 1.087000, 1.086630, 1.086650, 1.086680, 1.086690, 1.086690, 1.086690,
                    1.086690, 1.086650,
                ];
                let low = [
                    1.087010, 1.087120, 1.087080, 1.087170, 1.087110, 1.087010, 1.087100, 1.087120, 1.087110,
                    1.087080, 1.087000, 1.086630, 1.086630, 1.086610, 1.086630, 1.086640, 1.086650, 1.086650,
                    1.086670, 1.086630,
                ];
                let close = [
                    1.087130, 1.087120, 1.087220, 1.087230, 1.087110, 1.087120, 1.087100, 1.087120, 1.087130,
                    1.087080, 1.087000, 1.086630, 1.086630, 1.086650, 1.086640, 1.086690, 1.086650, 1.086690,
                    1.086670, 1.086640,
                ];

                let (atr_values, begin) = $fn_name(&high, &low, &close).unwrap();

                // print values
                for (index, value) in atr_values.iter().enumerate() {
                    println!("index {} = {}", begin + index + 1, value);
                }
            }
        });
    };
}

macro_rules! define_values_period_fn {
    ($(#[$attr:meta])* => $fn_name:ident, $ta_fn_name:ident) => {
        $(#[$attr])*
        pub fn $fn_name(
            input: &[f64],
            period: Option<usize>,
        ) -> Result<(Vec<f64>, usize), Error> {
            assert!(!input.is_empty());

            let mut out: Vec<f64> = Vec::with_capacity(input.len());
            let mut out_begin = MaybeUninit::<i32>::uninit();
            let mut out_size = MaybeUninit::<i32>::uninit();

            unsafe {
                let ret_code = ta::$ta_fn_name(
                    0,
                    (input.len() - 1) as _,
                    input.as_ptr(),
                    if let Some(period) = period {
                        period as _
                    } else {
                        ta::TA_INTEGER_DEFAULT
                    },
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
                        "Could not compute function; error: {:?}",
                        ret_code
                    ))),
                }
            }
        }

        concat_idents!(test_name = test, $fn_name {
            #[test]
            fn test_name() {
                let close_prices = [
                    1.087010, 1.087120, 1.087080, 1.087170, 1.087110, 1.087010, 1.087100, 1.087120, 1.087110,
                    1.087080, 1.087000, 1.086630, 1.086630, 1.086610, 1.086630, 1.086640, 1.086650, 1.086650,
                    1.086670, 1.086630,
                ];

                // compute sma, since we use a period of 10, the first 10 closes won't have
                // an sma value because there is not enough data, so begin will be set to
                // the index 29
                let (sma_values, begin) = $fn_name(&close_prices, Some(10)).unwrap();

                // print values
                for (index, value) in sma_values.iter().enumerate() {
                    println!("Close index {} = {}", begin + index + 1, value);
                }
            }
        });
    };
}
