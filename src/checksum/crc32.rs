/************************************************
* File: crc32.rs
* Author: Michal Švrček
*
* Crc 32 operations.
*
* ver. 0.1.0
*************************************************/

pub fn crc32(data: &[u8]) -> u32 {
    let mut crc: u32 = 0xFFFFFFFF;

    for &byte in data {
        crc ^= byte as u32;

        for _ in 0..8 {
            let lsb = crc & 1;

            crc >>= 1;

            if lsb == 1 {
                crc ^= 0xEDB88320
            }
        }
    }

    crc ^= 0xFFFFFFFF;

    crc
}

#[test]
fn test_crc32() {
    let data = b"123456789";

    assert_eq!(crc32(data), 0xCBF43926);
}
