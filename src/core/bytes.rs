/************************************************
* File: main.rs
* Author: Michal Švrček
*
* Byte operations.
*
* ver. 0.1.0
*************************************************/
use std::fmt::Write;

use crate::core::errors::HexError;

pub fn parse_hex(input: &str) -> Result<Vec<u8>, HexError> {
    let input = input.trim();

    if input.is_empty() {
        // empty input handler
        return Err(HexError::EmptyInput);
    }

    let normalized: String = input
        .replace("0x", "")
        .replace("0X", "")
        .chars()
        .filter(|c| !c.is_ascii_whitespace() && *c != ':')
        .collect();

    decode_hex(&normalized)
}

pub fn decode_hex(s: &str) -> Result<Vec<u8>, HexError> {
    if let Some(invalid) = s.chars().find(|c| !c.is_ascii_hexdigit()) {
        return Err(HexError::InvalidCharacter(invalid)); // return error for invalid chars
    }

    if s.len() % 2 == 1 {
        // if string is odd length
        return Err(HexError::OddLength);
    }

    let mut bytes = Vec::new(); // assign new vector for resulting bytes

    for i in (0..s.len()).step_by(2) {
        let pair = &s[i..i + 2]; // make a pair of bytes

        let value = u8::from_str_radix(pair, 16).map_err(|_| HexError::InvalidCharacter('?'))?; //parse to 8 bit num

        bytes.push(value);
    }

    Ok(bytes) // return final vector with bytes
}

pub fn encode_hex(bytes: &[u8]) -> String {
    let mut result = String::with_capacity(bytes.len() * 2);

    for &byte in bytes {
        write!(&mut result, "{byte:02X}").expect("Writing to String cannot fail"); // take uppercase letters
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_hex() {
        // test encoding
        let bytes = [0x41, 0x20, 0x00, 0xFF];

        assert_eq!(encode_hex(&bytes), "412000FF")
    }

    #[test]
    fn test_encode_empty() {
        // test empty string
        assert_eq!(encode_hex(&[]), "")
    }
}
