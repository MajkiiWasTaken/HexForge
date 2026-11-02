/************************************************
* File: error.rs
* Author: Michal Švrček
*
* Error definitions.
*
* ver. 0.1.0
*************************************************/

#[derive(Debug)]
pub enum HexError {
    EmptyInput,             // empty
    OddLength,              // odd number of chars e.g. "ABC"
    InvalidCharacter(char), // invalid char e.g. 'G'
}

impl std::fmt::Display for HexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyInput => write!(f, "HEX input is empty"),
            Self::OddLength => write!(f, "Odd number of HEX digits"),
            Self::InvalidCharacter(c) => {
                write!(f, "Invalid HEX character: '{c}'")
            }
        }
    }
}
