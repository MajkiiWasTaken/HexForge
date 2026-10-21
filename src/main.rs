/************************************************
* File: main.rs
* Author: Michal Švrček
*
* HexForge command line interface.
*
* ver. 0.1.0
*************************************************/
mod core;
mod output;

use crate::core::numeric::NumberBase;
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
        /// Path to the binary file
        path: String,
    },

    /// Convert a number between numeral systems
    Convert {
        /// Number to convert
        value: String,

        #[arg(long, value_enum, default_value_t = NumberBase::Dec)]
        from: NumberBase,
    },
}

fn run() -> Result<(), String> {
    let cli = Cli::parse();
    output::banner();

    match cli.command {
        Commands::Dump { path } => {
            println!("Dumping file: {path}");
            Ok(())
        }

        Commands::Convert { value, from } => {
            let number = core::numeric::parse_number(&value, from)?;

            println!("Decimal:     {number}");
            println!("Hexadecimal: 0x{number:X}");
            println!("Binary:      0b{number:b}");
            println!("Octal:       0o{number:o}");

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
