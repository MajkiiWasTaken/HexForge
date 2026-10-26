/************************************************
* File: dump.rs
* Author: Michal Švrček
*
* File dumps.
*
* ver. 0.1.0
*************************************************/
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;
use core::panic;

use crate::core::bytes;

pub fn dump_file(path: &Path, offset: Option<u64>, length: Option<u64>) -> Result<(), String> {
    let mut file = File::open(path).map_err(|e| format!("Cannot open file: {e}"))?;

    let mut buffer = [0u8; 16];

    let start = offset.unwrap_or(0);
    file.seek(SeekFrom::Start(start))
        .map_err(|e| format!("Cannot seek file: {e}"))?;

    let mut off = start;
    let mut remaining = length;

    loop {
        if remaining == Some(0) {
            break;
        }

        let to_read = match remaining {
            Some(bytes) => bytes.min(buffer.len() as u64) as usize,
            None => buffer.len(),
        };

        let n = file
            .read(&mut buffer[..to_read])
            .map_err(|e| format!("Cannot read file: {e}"))?;

        if n == 0 {
            break;
        }

        print!("{off:08X}|  ");

        // HEX representation
        for byte in &buffer[..n] {
            print!("{byte:02X} ");
        }

        // Align ASCII column
        for _ in n..16 {
            print!("   ");
        }

        // ASCII representation
        print!(" |");

        for &byte in &buffer[..n] {
            let character = if (0x20..=0x7E).contains(&byte) {
                byte as char
            } else {
                '.'
            };

            print!("{character}");
        }

        println!("|");

        if let Some(bytes) = &mut remaining {
            *bytes -= n as u64;
        };

        off += n as u64;
    }

    Ok(())
}
