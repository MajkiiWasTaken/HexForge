/************************************************
* File: crc16.rs
* Author: Michal Švrček
*
* Crc 16 operations.
*
* ver. 0.1.0
*************************************************/

pub fn xor8(data: &[u8]) -> u8 {
    let mut crc = 0;

    for &byte in data {
        crc ^= byte;
    }

    crc
}

#[test]
fn test_xor8() {
    assert_eq!(xor8(&[0x12, 0x34, 0x56]), 0x70);
}
