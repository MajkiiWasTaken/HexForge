/************************************************
* File: main.rs
* Author: Michal Švrček
*
* HexForge command line interface.
*
* ver. 0.1.0
*************************************************/
mod binary;
mod checksum;
mod core;
mod modbus;
mod output;

use crate::{
    core::{
        codec::DataType,
        endian::ByteOrder,
        numeric::{NumberBase, parse_cli_number},
    },
    modbus::order::{RegisterOrder, split_registers},
};
use clap::{Parser, Subcommand};
use std::process::ExitCode;

#[derive(Parser)]
#[command(
    name = "hexforge",
    version,
    about = "Cross-platform data diagnostics",
    arg_required_else_help = true
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Display a hexadecimal dump of a file
    Dump {
        path: std::path::PathBuf,

        #[arg(long, value_parser = parse_cli_number)]
        offset: Option<u64>,

        #[arg(long, value_parser = parse_cli_number)]
        length: Option<u64>,
    },

    /// Convert a number between numeral systems
    Convert {
        /// Number to convert
        value: String,

        #[arg(long, value_enum, default_value_t = NumberBase::Dec)]
        from: NumberBase,
    },

    Decode {
        // Hex bytes to decode
        value: String,

        //Target data type
        #[arg(long = "type")]
        data_type: DataType,

        // Byte order
        #[arg(long, value_enum, default_value_t = ByteOrder::Big)]
        endian: ByteOrder,

        // Register order
        #[arg(long, value_enum, default_value_t = RegisterOrder::ABCD)]
        order: RegisterOrder,
    },

    Encode {
        // Hex bytes to encode
        value: String,

        //Target data type
        #[arg(long = "type")]
        data_type: DataType,

        // Byte order
        #[arg(long, value_enum, default_value_t = ByteOrder::Big)]
        endian: ByteOrder,

        // Register order
        #[arg(long, value_enum, default_value_t = RegisterOrder::ABCD)]
        order: RegisterOrder,
    },

    Crc {
        #[command(subcommand)]
        algorithm: CrcCommands,
    },

    Search {
        /// Binary file to search
        path: std::path::PathBuf,

        /// HEX byte sequence to find
        pattern: String,
    },

    Compare {
        /// BINS
        file1: std::path::PathBuf,
        file2: std::path::PathBuf,
    },

    Inspect {
        /// Path to the binary file
        path: std::path::PathBuf,
    },
}

#[derive(Subcommand)]
enum CrcCommands {
    Modbus {
        /// HEX bytes used for CRC calculation
        value: String,
    },

    CCITT {
        value: String,
    },

    Crc32 {
        value: String,
    },

    Xor {
        value: String,
    },
}

fn run() -> Result<(), String> {
    let cli = Cli::parse();
    output::banner();

    match cli.command {
        Commands::Dump {
            path,
            offset,
            length,
        } => {
            println!("Dumping file: {}", path.display());
            binary::dump::dump_file(&path, offset, length)
        }

        Commands::Convert { value, from } => {
            let number = core::numeric::parse_number(&value, from)?;

            println!("Decimal:     {number}");
            println!("Hexadecimal: 0x{number:X}");
            println!("Binary:      0b{number:b}");
            println!("Octal:       0o{number:o}");

            Ok(())
        }

        Commands::Decode {
            value,
            data_type,
            endian,
            order,
        } => {
            // Parse HEX input into bytes
            let bytes =
                core::bytes::parse_hex(&value).map_err(|e| format!("Invalid HEX input: {e}"))?;

            // Validate byte count
            if bytes.len() != data_type.byte_size() {
                return Err(format!(
                    "Expected {} bytes, got {}",
                    data_type.byte_size(),
                    bytes.len()
                ));
            }

            // Apply register order for 32-bit values
            let bytes = if bytes.len() == 4 {
                let array: [u8; 4] = bytes
                    .as_slice()
                    .try_into()
                    .map_err(|_| "Expected exactly 4 bytes".to_string())?;

                modbus::order::reorder_bytes(array, order).to_vec()
            } else {
                bytes
            };

            // Decode using selected endian
            let result = core::codec::decode(&bytes, data_type, endian)?;

            println!("Decoded value: {result}");

            Ok(())
        }

        Commands::Encode {
            value,
            data_type,
            endian,
            order,
        } => {
            // Encode numeric value into raw bytes
            let bytes = core::codec::encode(&value, data_type, endian)?;

            // Apply Modbus register order
            let bytes = if bytes.len() == 4 {
                let array: [u8; 4] = bytes
                    .as_slice()
                    .try_into()
                    .map_err(|_| "Expected exactly 4 bytes".to_string())?;

                let registers = split_registers(array);

                println!("Register 1: {}", registers[0]);
                println!("Register 2: {}", registers[1]);

                modbus::order::reorder_bytes(array, order).to_vec()
            } else {
                bytes
            };

            // Convert bytes to HEX string
            let hex = core::bytes::encode_hex(&bytes);

            println!("HEX: {hex}");

            Ok(())
        }

        Commands::Search { path, pattern } => {
            // Parse HEX pattern
            let pattern_bytes = core::bytes::parse_hex(&pattern)
                .map_err(|e| format!("Invalid HEX pattern: {e:?}"))?;

            // Read binary file
            let data = std::fs::read(&path).map_err(|e| format!("Cannot read file: {e}"))?;

            // Search for matching byte sequences
            let matches = binary::search::search_bytes(&data, &pattern_bytes);

            for offset in &matches {
                println!("Match at offset: 0x{offset:08X}");
            }

            println!("Total matches: {}", matches.len());

            Ok(())
        }

        Commands::Crc { algorithm } => match algorithm {
            CrcCommands::Modbus { value } => {
                let bytes = core::bytes::parse_hex(&value)
                    .map_err(|e| format!("Invalid HEX input: {e}"))?;

                let crc = checksum::crc16::crc16_modbus(&bytes);

                let crc_bytes = crc.to_le_bytes();

                println!("CRC: 0x{crc:04X}");

                println!("CRC bytes: {:02X} {:02X}", crc_bytes[0], crc_bytes[1]);

                Ok(())
            }

            CrcCommands::CCITT { value } => {
                let bytes = core::bytes::parse_hex(&value)
                    .map_err(|e| format!("Invalid HEX input: {e}"))?;

                let crc = checksum::crc16::crc16_ccitt(&bytes);

                let crc_bytes = crc.to_be_bytes();

                println!("CRC: 0x{crc:04X}");

                println!("CRC bytes: {:02X} {:02X}", crc_bytes[0], crc_bytes[1]);

                Ok(())
            }

            CrcCommands::Crc32 { value } => {
                let bytes = core::bytes::parse_hex(&value)
                    .map_err(|e| format!("Invalid HEX input: {e}"))?;

                let crc = checksum::crc32::crc32(&bytes);

                let crc_bytes = crc.to_le_bytes();

                println!("CRC: 0x{crc:04X}");

                println!(
                    "CRC bytes: {:02X} {:02X} {:02X} {:02X}",
                    crc_bytes[0], crc_bytes[1], crc_bytes[2], crc_bytes[3]
                );

                Ok(())
            }

            CrcCommands::Xor { value } => {
                let bytes = core::bytes::parse_hex(&value)
                    .map_err(|e| format!("Invalid HEX input: {e}"))?;

                let crc = checksum::crc8::xor8(&bytes);

                let crc_bytes = crc.to_be_bytes();

                println!("CRC: 0x{crc:X}");

                println!("CRC bytes: {:02X}", crc_bytes[0]);

                Ok(())
            }
        },

        Commands::Compare { file1, file2 } => {
            // Read both files
            let a = std::fs::read(&file1).map_err(|e| format!("Cannot read first file: {e}"))?;

            let b = std::fs::read(&file2).map_err(|e| format!("Cannot read second file: {e}"))?;

            // Compare binary data
            let differences = binary::compare::compare_bytes(&a, &b);

            println!("Comparing:");
            println!("File 1: {}", file1.display());
            println!("File 2: {}", file2.display());

            println!();
            println!("{:<12} | {:>6} | {:>6} |", "Offset", "File 1", "File 2");
            println!("{}", "-".repeat(32));

            for &offset in &differences {
                let left = a.get(offset);
                let right = b.get(offset);

                println!(
                    "{:<12} | {:>6} | {:>6} |",
                    format!("0x{offset:08X}"),
                    left.map(|b| format!("{b:02X}"))
                        .unwrap_or_else(|| "--".to_string()),
                    right
                        .map(|b| format!("{b:02X}"))
                        .unwrap_or_else(|| "--".to_string())
                );
            }

            println!("{}", "-".repeat(32));
            println!();
            println!("Total differences: {}", differences.len());

            Ok(())
        }

        Commands::Inspect { path } => binary::inspect::inspect_file(&path),
    }
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            output::error(&error);
            ExitCode::FAILURE
        }
    }
}
