/************************************************
* File: main.rs
* Author: Michal Švrček
*
* Registers related operations.
*
* ver. 0.3.0
*************************************************/
use clap::ValueEnum;

use crate::modbus::order;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum RegisterOrder {
    ABCD, // 41 20 00 00
    BADC, // 20 41 00 00
    CDAB, // 00 00 41 20
    DCBA, // 00 00 20 41
}

pub fn reorder_bytes(bytes: [u8; 4], order: RegisterOrder) -> [u8; 4] {
    match order {
        RegisterOrder::ABCD => bytes,
        RegisterOrder::BADC => [bytes[1], bytes[0], bytes[3], bytes[2]],
        RegisterOrder::CDAB => [bytes[2], bytes[3], bytes[0], bytes[1]],
        RegisterOrder::DCBA => [bytes[3], bytes[2], bytes[1], bytes[0]],
    }
}

pub fn split_registers(bytes: [u8; 4]) -> [u16; 2] {
    [
        u16::from_be_bytes([bytes[0], bytes[1]]),
        u16::from_be_bytes([bytes[2], bytes[3]]),
    ]
}

pub fn join_registers(registers: [u16; 2]) -> [u8; 4] {
    let first = registers[0].to_be_bytes();
    let second = registers[1].to_be_bytes();

    [
        first[0],
        first[1],
        second[0],
        second[1],
    ]
}