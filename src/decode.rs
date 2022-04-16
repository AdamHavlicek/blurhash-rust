use std::f64::consts::PI;

use crate::base_83::decode83;
use crate::errors::DecodeError;
use crate::types::NumberTriplet;
use crate::utils::{linear2srgb, sign_pow, srgb2linear};

fn decode_dc(value: usize) -> NumberTriplet {
    [value >> 16, (value >> 8) & 255, value & 255].map(srgb2linear)
}

fn decode_ac(value: usize, max_value: f64) -> NumberTriplet {
    let quant_rgb = [
        f64::floor(value as f64 / (19. * 19.)),
        f64::floor(value as f64 / 19.) % 19.,
        value as f64 % 19.,
    ];

    quant_rgb.map(|v| sign_pow((v - 9.) / 9., 2.) * max_value)
}

pub fn decode(hash: &str, width: usize, height: usize) -> Result<Vec<u8>, DecodeError> {
    if hash.len() < 6 {
        return Err(DecodeError::InvalidLength)
    }

    // TODO: proper error handling and hash validation
    let size_flag = decode83(hash.get(0..1).unwrap());
    let num_y = (size_flag / 9) + 1;
    let num_x = (size_flag % 9) + 1;

    let expected_digits = 4 + 2 * num_x * num_y;
    if hash.len() != expected_digits {
        return Err(DecodeError::LengthMismatch)
    }

    let quantised_max_value = decode83(hash.get(1..2).unwrap());
    let max_value = (quantised_max_value + 1) as f64 / 166.;

    let colors_capacity = num_x * num_y;
    let mut colors: Vec<NumberTriplet> = Vec::with_capacity(colors_capacity);

    colors.push(decode_dc(decode83(hash.get(2..6).unwrap())));
    for index in 1..colors_capacity {
        let range = (4 + index * 2)..(4 + index * 2 + 2);
        colors.push(
            decode_ac(
                decode83(hash.get(range).unwrap()),
                max_value * 1.,
            )
        );
    }

    let bytes_per_row = width * 4;
    let mut pixels: Vec<u8> = vec![0; bytes_per_row * height];

    for y in 0..height {
        for x in 0..width {
            let mut rgba = [0., 0., 0., 255.];

            for j in 0..num_y {
                for i in 0..num_x {
                    let basis = f64::cos(PI * (x as f64) * (i as f64) / (width as f64))
                        * f64::cos(PI * (y as f64) * (j as f64) / (height as f64));
                    let color = colors[i + j * num_x];

                    for rgb_index in 0..3 {
                        rgba[rgb_index] += color[rgb_index] * basis;
                    }
                }
            }

            for rgba_index in 0..4 {
                let value: u8;
                if rgba_index == 3 {
                    value = rgba[rgba_index] as u8;
                } else {
                    value = linear2srgb(rgba[rgba_index]) as u8;
                }
                pixels[4 * x + rgba_index + y * bytes_per_row] = value;
            }
        }
    }
    
    Ok(pixels.into_iter().collect())
}

#[cfg(test)]
mod tests {
    use super::decode;
    use image;

    #[test]
    fn test_decode() {
        // Arrange
        let hash = "LUDT3yayV?ay%jWBa#a}9Xj[j@fP";
        let path = "demo/decode-test-expected.png";
        let expected = image::open(path).unwrap().to_rgba8();
        let (width, height) = expected.dimensions();

        // Act
        let result = decode(
            hash,
            usize::try_from(width).unwrap(),
            usize::try_from(height).unwrap(),
        );
        // image::save_buffer(
        //     path,
        //     &result.unwrap(),
        //     40,
        //     30,
        //     image::ColorType::Rgba8,
        // )
        // .unwrap();

        // Assert
        match result {
            Ok(result) => assert_eq!(result, expected.into_vec()),
            Err(_err) => assert!(false),
        }
    }
}
