/************************************************
* File: search.rs
* Author: Michal Švrček
*
* Compare operations.
*
* ver. 0.5.1
*************************************************/

pub fn compare_bytes(a: &[u8], b: &[u8]) -> Vec<usize> {
    let mut differences = Vec::new();

    // Compare bytes at matching positions
    for (index, (&left, &right)) in a.iter().zip(b.iter()).enumerate() {
        if left != right {
            differences.push(index);
        }
    }

    // Include bytes that exist only in the longer input
    if a.len() != b.len() {
        let min_len = a.len().min(b.len());
        let max_len = a.len().max(b.len());

        for index in min_len..max_len {
            differences.push(index);
        }
    }

    differences
}

#[test]
fn test_compare_bytes() {
    let a = [0x12, 0x34, 0xAB, 0xCD];
    let b = [0x12, 0x34, 0xFF, 0xCD];

    assert_eq!(compare_bytes(&a, &b), vec![2]);
}

#[test]
fn test_compare_different_lengths() {
    let a = [0x12, 0x34];
    let b = [0x12, 0x34, 0x56];

    assert_eq!(compare_bytes(&a, &b), vec![2]);
}
