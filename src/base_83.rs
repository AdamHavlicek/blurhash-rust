use crate::consts::DIGIT_CHARACTERS;

pub fn decode83(string: &str) -> usize {
    let mut result: usize = 0;

    for char_ in string.chars() {
        let index = DIGIT_CHARACTERS
            .iter()
            .position(|&element| element == char_)
            .unwrap();
        result = result * 83 + index;
    }

    result
}

pub fn encode83(number: usize, length: u32) -> String {
    let mut result = String::with_capacity(length as usize);

    for i in 1..=length {
        let index = (number / usize::pow(83, length - i)) % 83;
        result.push(DIGIT_CHARACTERS[index]);
    }

    result 
}

#[cfg(test)]
mod tests {
    use super::{
        decode83,
        encode83
    };

    #[test]
    fn test_decode83() {
        // Arrange
        let string = "1";
        let expected_result = 1;

        // Act
        let result = decode83(&string);

        // Assert
        assert_eq!(result, expected_result);
        
    }

    #[test]
    fn test_encode83() {
        // Arrange
        let number = 0;
        let length = 2;
        let expected_result = String::from("00");

        // Act
        let result = encode83(number, length);

        // Assert
        assert_eq!(result, expected_result);
        
    }
}
