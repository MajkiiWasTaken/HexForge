/************************************************
* File: main.rs
* Author: Michal Švrček
*
* Error definitions.
*
* ver. 0.1.0
*************************************************/

pub enum HexError {
    EmptyInput,             // empty
    OddLength,              // odd number of chars e.g. "ABC"
    InvalidCharacter(char), // invalid char e.g. 'G'
}
