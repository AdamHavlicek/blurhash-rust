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

