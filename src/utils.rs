
pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub fn srgb2linear(number: usize) -> f64 {
    let mut result = (number as f64) / 255. ;

    if result <= 0.04045 {
        result /= 12.92;
    } else {
        result = ((result + 0.055) / 1.055).powf(2.4);
    }
    
    result
}

pub fn linear2srgb(number: f64) -> usize {
    let value = f64::max(0., f64::min(1., number));

    if value <= 0.003_130_8 {
        f64::round(value * 12.92 * 255. + 0.5) as usize
    } else {
        f64::round((1.055 * f64::powf(value, 1. / 2.4) - 0.055) * 255. + 0.5) as usize
    }
}

pub fn sign_pow(val: f64, exp: f64) -> f64 {
    val.abs().powf(exp).copysign(val)
}
