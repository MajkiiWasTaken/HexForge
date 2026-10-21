use core::panic;
/************************************************
* File: main.rs
* Author: Michal Švrček
*
* File dumps.
*
* ver. 0.1.0
*************************************************/
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn dump_file(path: &Path) -> Result<(), String> {
    let mut file = File::open(path)
        .map_err(|e| format!("Cannot open file: {e}"))?;

    let mut buffer = [0u8; 16];

    let mut offset: u64 = 0;

    loop {
        let n = file
            .read(&mut buffer)
            .map_err(|e| format!("Cannot read file: {e}"))?;

        if n == 0 {
            break;
        }

        print!("{offset:08X}  ");

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

        offset += n as u64;
    }

    Ok(())
}