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
