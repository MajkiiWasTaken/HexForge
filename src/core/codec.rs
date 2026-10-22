/************************************************
* File: main.rs
* Author: Michal Švrček
*
* Codec enums and functions.
*
* ver. 0.1.0
*************************************************/

use crate::core::endian::ByteOrder;
use clap::ValueEnum;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum DataType {
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    U64,
    I64,
    F32,
    F64,
}

impl DataType {
    pub fn byte_size(&self) -> usize {
        match self {
            DataType::U8 | DataType::I8 => 1,
            DataType::U16 | DataType::I16 => 2,
            DataType::U32 | DataType::I32 | DataType::F32 => 4,
            DataType::U64 | DataType::I64 | DataType::F64 => 8,
        }
    }
}

pub fn decode(bytes: &[u8], data_type: DataType, endian: ByteOrder) -> Result<String, String> {
    // Validate byte count
    if bytes.len() != data_type.byte_size() {
        return Err(format!(
            "Expected {} bytes, got {}",
            data_type.byte_size(),
            bytes.len()
        ));
    }

    let result = match data_type {
        DataType::U8 => bytes[0].to_string(),

        DataType::I8 => (bytes[0] as i8).to_string(),

        DataType::U16 => {
            let array: [u8; 2] = bytes.try_into().map_err(|_| "Invalid u16 bytes")?;

            match endian {
                ByteOrder::Big => u16::from_be_bytes(array),
                ByteOrder::Little => u16::from_le_bytes(array),
            }
            .to_string()
        }

        DataType::I16 => {
            let array: [u8; 2] = bytes.try_into().map_err(|_| "Invalid i16 bytes")?;

            match endian {
                ByteOrder::Big => i16::from_be_bytes(array),
                ByteOrder::Little => i16::from_le_bytes(array),
            }
            .to_string()
        }

        DataType::U32 => {
            let array: [u8; 4] = bytes.try_into().map_err(|_| "Invalid u32 bytes")?;

            match endian {
                ByteOrder::Big => u32::from_be_bytes(array),
                ByteOrder::Little => u32::from_le_bytes(array),
            }
            .to_string()
        }

        DataType::I32 => {
            let array: [u8; 4] = bytes.try_into().map_err(|_| "Invalid i32 bytes")?;

            match endian {
                ByteOrder::Big => i32::from_be_bytes(array),
                ByteOrder::Little => i32::from_le_bytes(array),
            }
            .to_string()
        }

        DataType::U64 => {
            let array: [u8; 8] = bytes.try_into().map_err(|_| "Invalid u64 bytes")?;

            match endian {
                ByteOrder::Big => u64::from_be_bytes(array),
                ByteOrder::Little => u64::from_le_bytes(array),
            }
            .to_string()
        }

        DataType::I64 => {
            let array: [u8; 8] = bytes.try_into().map_err(|_| "Invalid i64 bytes")?;

            match endian {
                ByteOrder::Big => i64::from_be_bytes(array),
                ByteOrder::Little => i64::from_le_bytes(array),
            }
            .to_string()
        }

        DataType::F32 => {
            let array: [u8; 4] = bytes.try_into().map_err(|_| "Invalid f32 bytes")?;

            match endian {
                ByteOrder::Big => f32::from_be_bytes(array),
                ByteOrder::Little => f32::from_le_bytes(array),
            }
            .to_string()
        }

        DataType::F64 => {
            let array: [u8; 8] = bytes.try_into().map_err(|_| "Invalid f64 bytes")?;

            match endian {
                ByteOrder::Big => f64::from_be_bytes(array),
                ByteOrder::Little => f64::from_le_bytes(array),
            }
            .to_string()
        }
    };

    Ok(result)
}
