/************************************************
* File: main.rs
* Author: Michal Švrček
*
* Endian operations.
*
* ver. 0.1.0
*************************************************/

#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)] // clap::ValueEnum lets us use bin, hex, dec as cli arguments 
pub enum ByteOrder {
    Big,
    Little,
}

pub fn decode_u16(bytes: [u8; 2], order: ByteOrder) -> u16 {
    match order {
        ByteOrder::Big => u16::from_be_bytes(bytes),
        ByteOrder::Little => u16::from_le_bytes(bytes),
    }
}

pub fn encode_u16(value: u16, order: ByteOrder) -> [u8; 2] {
    match order {
        ByteOrder::Big => value.to_be_bytes(),
        ByteOrder::Little => value.to_le_bytes(),
    }
}

pub fn decode_u32(bytes: [u8; 4], order: ByteOrder) -> u32 {
    match order {
        ByteOrder::Big => u32::from_be_bytes(bytes),
        ByteOrder::Little => u32::from_le_bytes(bytes),
    }
}

pub fn encode_u32(value: u32, order: ByteOrder) -> [u8; 4] {
    match order {
        ByteOrder::Big => value.to_be_bytes(),
        ByteOrder::Little => value.to_le_bytes(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_u16() {
        assert_eq!(decode_u16([0x12, 0x34], ByteOrder::Big), 4660);
        assert_eq!(decode_u16([0x12, 0x34], ByteOrder::Little), 13330);
    }

    #[test]
    fn test_encode_u16() {
        assert_eq!(encode_u16(4660, ByteOrder::Big), [0x12, 0x34]);
        assert_eq!(encode_u16(4660, ByteOrder::Little), [0x34, 0x12]);
    }

    #[test]
    fn test_decode_u32() {
        let bytes = [0x12, 0x34, 0x56, 0x78];

        assert_eq!(decode_u32(bytes, ByteOrder::Big), 0x12345678);
        assert_eq!(decode_u32(bytes, ByteOrder::Little), 0x78563412);
    }

    #[test]
    fn test_encode_u32() {
        let value = 0x12345678;

        assert_eq!(encode_u32(value, ByteOrder::Big), [0x12, 0x34, 0x56, 0x78]);

        assert_eq!(
            encode_u32(value, ByteOrder::Little),
            [0x78, 0x56, 0x34, 0x12]
        );
    }
}
