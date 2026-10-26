/************************************************
* File: output.rs
* Author: Michal Švrček
*
* Console output and structured diagnostic results.
*
* ver. 0.5.0
*************************************************/

use colored::Colorize;

pub fn banner() {
    println!(
        "{}",
        format!("◆ HexForge v{}", env!("CARGO_PKG_VERSION"))
            .bright_blue()
            .bold()
    );
    println!("{}", "────────────────────────────────────────".dimmed());
}

// pub fn heading(message: &str) {
//     println!("\n{}", message.bright_cyan().bold());
// }

// pub fn info(message: &str) {
//     println!("{} {message}", "[INFO]".blue().bold());
// }

pub fn error(message: &str) {
    eprintln!("{} {message}", "[ERROR]".red().bold());
}
