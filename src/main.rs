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

use crate::core::{codec::DataType, endian::ByteOrder, numeric::{NumberBase, parse_cli_number}};
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
    }
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

        Commands::Decode {value, data_type, endian,} => {
            // Parse HEX string into bytes
            let bytes = core::bytes::parse_hex(&value)
                .map_err(|e| format!("Invalid HEX input: {e:?}"))?;

            //Validate byte count
            if bytes.len() != data_type.byte_size() {
                return Err(format!(
                    "Invalid byte count: expected {}, got {}",
                    data_type.byte_size(),
                    bytes.len()
                ));
            }

            // Decode according to selected data type
            match data_type {
                DataType::U16 => {
                    let array: [u8; 2] = bytes
                        .as_slice()
                        .try_into()
                        .map_err(|_| "Expected exactly 2 bytes".to_string())?;

                    let result = core::endian::decode_u16(array, endian);

                    println!("Value: {result}");
                }

                _ => {
                    return Err(format!(
                        "Decoding {:?} is not implemented yet",
                        data_type
                    ));
                }
            }

            Ok(())
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
