/************************************************
* File: crc16.rs
* Author: Michal Švrček
*
* Crc 16 operations.
*
* ver. 0.1.0
*************************************************/


pub fn crc16_modbus(data: &[u8]) -> u16 {
    let mut crc: u16 = 0xFFFF;

    for &byte in data {
        crc ^= byte as u16;

        for _ in 0..8 {
            // Save the least significant bit before shifting.
            let lsb = crc & 1;

            // Shift CRC right by one bit for every processed bit.
            crc >>= 1;

            // If the original LSB was 1, XOR with the reflected polynomial.
            if lsb == 1 {
                crc ^= 0xA001;
            }
        }
    }

    crc
}

#[test]
fn test_crc16_modbus() {
    let data = [0x01, 0x03, 0x00, 0x00, 0x00, 0x02];

    assert_eq!(crc16_modbus(&data), 0x0BC4);
}