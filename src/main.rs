/************************************************
* File: main.rs
* Author: Michal Švrček
*
* HexForge command line interface.
*
* ver. 0.1.0
*************************************************/
mod binary;
mod core;
mod output;
mod modbus;
mod checksum;

use crate::{core::{
    codec::DataType,
    endian::ByteOrder,
    numeric::{NumberBase, parse_cli_number},
}, modbus::order::{RegisterOrder, split_registers}};
use clap::{Parser, Subcommand};
use std::{process::ExitCode, result};

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
}

#[derive(Subcommand)]
enum CrcCommands {
    Modbus {
        /// HEX bytes used for CRC calculation
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
            let bytes = core::bytes::parse_hex(&value)
                .map_err(|e| format!("Invalid HEX input: {e:?}"))?;

            let array: [u8; 4] = bytes
                .as_slice()
                .try_into()
                .map_err(|_| "Expected exactly 4 bytes".to_string())?;

            let reordered = modbus::order::reorder_bytes(array, order);

            let result = core::codec::decode(
                &reordered,
                data_type,
                ByteOrder::Big,
            )?;

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

        Commands::Crc { algorithm } => {
            match algorithm {
                CrcCommands::Modbus { value } => {
                    let bytes = core::bytes::parse_hex(&value) .map_err(|e| format!("Invalid hex input: {e:?}"))?;

                    let crc = checksum::crc16::crc16_modbus(&bytes);

                    println!("CRC: 0x{crc:04X}");

                    Ok(())
                }
            }
        }
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
