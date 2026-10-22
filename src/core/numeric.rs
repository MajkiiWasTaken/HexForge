/************************************************
* File: main.rs
* Author: Michal Švrček
*
* Numeric operations.
*
* ver. 0.1.0
*************************************************/

#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)] // clap::ValueEnum lets us use bin, hex, dec as cli arguments 
pub enum NumberBase {
    Bin,
    Hex,
    Dec,
    Oct,
}

impl NumberBase {
    pub fn radix(self) -> u32 {
        match self {
            NumberBase::Bin => 2,
            NumberBase::Oct => 8,
            NumberBase::Dec => 10,
            NumberBase::Hex => 16,
        }
    }
}

pub fn parse_number(input: &str, base: NumberBase) -> Result<u64, String> {
    let cleaned = input.trim();

    let cleaned = match base {
        NumberBase::Bin => cleaned
            .strip_prefix("0b")
            .or_else(|| cleaned.strip_prefix("0B"))
            .unwrap_or(cleaned),

        NumberBase::Hex => cleaned
            .strip_prefix("0x")
            .or_else(|| cleaned.strip_prefix("0X"))
            .unwrap_or(cleaned),

        NumberBase::Oct => cleaned
            .strip_prefix("0o")
            .or_else(|| cleaned.strip_prefix("0O"))
            .unwrap_or(cleaned),

        NumberBase::Dec => cleaned,
    };

    u64::from_str_radix(cleaned, base.radix()).map_err(|error| format!("Invalid number: {error}"))
}

pub fn parse_cli_number(input: &str) -> Result<u64, String> {
    let input = input.trim();

    let base = if input.starts_with("0x") || input.starts_with("0X") {
        NumberBase::Hex
    } else if input.starts_with("0b") || input.starts_with("0B") {
        NumberBase::Bin
    } else {
        NumberBase::Dec
    };

    parse_number(input, base)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decimal() {
        assert_eq!(parse_number("255", NumberBase::Dec), Ok(255));
    }

    #[test]
    fn test_hexadecimal() {
        assert_eq!(parse_number("FF", NumberBase::Hex), Ok(255));
        assert_eq!(parse_number("0xFF", NumberBase::Hex), Ok(255));
    }

    #[test]
    fn test_binary() {
        assert_eq!(parse_number("11111111", NumberBase::Bin), Ok(255));
    }

    #[test]
    fn test_invalid_number() {
        assert!(parse_number("GG", NumberBase::Hex).is_err());
    }

    #[test]
    fn test_overflow() {
        assert!(parse_number("18446744073709551616", NumberBase::Dec).is_err());
    }

    #[test]
    fn test_cli_number_parser() {
        assert_eq!(parse_cli_number("256"), Ok(256));
        assert_eq!(parse_cli_number("0x100"), Ok(256));
        assert_eq!(parse_cli_number("0X100"), Ok(256));
        assert_eq!(parse_cli_number("0b100000000"), Ok(256));
        assert_eq!(parse_cli_number("0B100000000"), Ok(256));
    }

    #[test]
    fn test_cli_number_invalid() {
        assert!(parse_cli_number("0xGG").is_err());
        assert!(parse_cli_number("-1").is_err());
        assert!(parse_cli_number("").is_err());
    }
}
