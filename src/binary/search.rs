/************************************************
* File: search.rs
* Author: Michal Švrček
*
* Search operations.
*
* ver. 0.5.0
*************************************************/

use std::io::empty;

pub fn search_bytes(data: &[u8], pattern: &[u8]) -> Vec<usize> {
    let mut matches = Vec::new();

    if pattern.is_empty() {
        return matches;
    }

    for (offset, window) in data.windows(pattern.len()).enumerate() {
        if window == pattern {
            matches.push(offset);
        }
    }

    matches
}

#[test]
fn test_search_bytes() {
    let data = [0x12, 0x34, 0xAB, 0xCD, 0x12, 0x34];
    let pattern = [0x12, 0x34];

    assert_eq!(search_bytes(&data, &pattern), vec![0, 4]);
}

#[test]
fn test_search_overlapping() {
    let data = [0xAA, 0xAA, 0xAA];
    let pattern = [0xAA, 0xAA];

    assert_eq!(search_bytes(&data, &pattern), vec![0, 1]);
}

#[test]
fn test_search_empty_pattern() {
    assert!(search_bytes(&[0x12, 0x34], &[]).is_empty());
}
