use std::f64::consts::PI;

use crate::base_83::encode83;
use crate::consts::BYTES_PER_PIXEL;
use crate::errors::EncodeError;
use crate::types::NumberTriplet;
use crate::utils::{linear2srgb, sign_pow, srgb2linear};


fn multiply_basis<F>(pixels: &[u8], width: usize, height: usize, basis_function: F) -> NumberTriplet
where
    F: Fn(f64, f64) -> f64,
{
    let bytes_per_row = width * BYTES_PER_PIXEL as usize;
    let mut rgb: [f64; 3] = [0.; 3];

    for x in 0..width {
        for y in 0..height {
            let basis = basis_function(x as f64, y as f64);
            for index in 0..rgb.len() {
                rgb[index] += basis
                    * srgb2linear(
                        usize::try_from(pixels[BYTES_PER_PIXEL * x + index + y * bytes_per_row])
                            .unwrap(),
                    );
            }
        }
    }
    let scale = 1. / ((width * height) as f64);

    rgb.map(|v| v * scale)
}

fn encode_dc(value: &NumberTriplet) -> usize {
    (linear2srgb(value[0]) << 16) + (linear2srgb(value[1]) << 8) + linear2srgb(value[2])
}

fn encode_ac(value: &NumberTriplet, max_value: f64) -> usize {
    let mut quant_rgb: [f64; 3] = [0.; 3];
    for index in 0..quant_rgb.len() {
        quant_rgb[index] = f64::floor(f64::max(
            0.,
            f64::min(
                18.,
                f64::floor(sign_pow(value[index] / max_value, 0.5) * 9. + 9.5),
            ),
        ));
    }
    (quant_rgb[0] * 19. * 19. + quant_rgb[1] * 19. + quant_rgb[2]) as usize
}

pub fn encode(
    pixels: Vec<u8>,
    width: usize,
    height: usize,
    component_x: usize,
    component_y: usize,
) -> Result<String, EncodeError> {
    if !(1..=9).contains(&component_x) || !(1..=9).contains(&component_y) {
        // panic!("BlurHash must have between 1 and 9 components");
        return Err(EncodeError::ComponentNumberOutbound)
    }
    if width * height * 4 != pixels.len() {
        // panic!("Width and height must match the pixels array");
        return Err(EncodeError::PixelArrayMismatch);
    }

    let mut factors: Vec<NumberTriplet> = Vec::new();
    for y in 0..component_y {
        for x in 0..component_x {
            let normalisation = if x == 0 && y == 0 { 1. } else { 2. };
            let factor = multiply_basis(&pixels, width, height, |i, j| {
                normalisation
                    * f64::cos((PI * x as f64 * i) / width as f64)
                    * f64::cos((PI * y as f64 * j) / height as f64)
            });
            factors.push(factor);
        }
    }
    let dc = factors[0];
    let ac = &factors[1..factors.len()];

    // capactiy size_flag, max_value, dc, ac 
    let string_capacity = 1 + 1 + 4 + ac.len() * 2;
    let mut hash = String::with_capacity(string_capacity);

    let size_flag = (component_x - 1) + (component_y - 1) * 9;
    hash.push_str(&encode83(size_flag, 1));

    let max_value: f64;
    if !ac.is_empty() {
        // INFO: kinda weird AF
        let actual_max_value: f64 = ac
            .iter()
            .map(|triplet| {
                triplet
                    .iter()
                    .map(|v| f64::abs(*v))
                    .reduce(f64::max)
                    .unwrap()
            })
            .reduce(f64::max)
            .unwrap();
        let quantised_max_value = usize::max(
            0,
            usize::min(82, f64::floor(actual_max_value * 166. - 0.5) as usize),
        );
        max_value = (quantised_max_value + 1) as f64 / 166.;
        hash.push_str(&encode83(quantised_max_value, 1));
    } else {
        max_value = 1.;
        hash.push_str(&encode83(0, 1));
    }

    hash.push_str(&encode83(encode_dc(&dc), 4));

    for factor in ac {
        hash.push_str(&encode83(encode_ac(factor, max_value), 2));
    }

    Ok(hash)
}

#[cfg(test)]
mod tests {
    use image;

    use super::encode;

    #[test]
    fn test_encode() {
        // Arrange
        let file = image::open("demo/test_image.jpeg")
            .expect("no file found")
            .to_rgba8();
        let (width, height) = file.dimensions();
        let pixels = file.into_vec();

        // INFO: hash taken from typescript version
        // let expected_str = String::from("UcGRVNIp%1t7~AIpt6oeE2I;R-niD+bcWXni");
        let expected_str = String::from("UcGRVOIp%1t7~AIpt6oeE2I;R-niD+bcWXni");

        // Act
        let result = encode(
            pixels,
            usize::try_from(width).unwrap(),
            usize::try_from(height).unwrap(),
            4,
            4,
        )
        .unwrap();

        // Assert
        assert_eq!(result, expected_str);
    }
}
