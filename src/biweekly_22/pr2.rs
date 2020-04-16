
use std::collections::HashMap;

fn max_number_of_families(n: i32, reserved_seats: Vec<Vec<i32>>) -> i32 {
    let mut number = 0;
    let mut map = HashMap::<i32, u16>::new();
    // Use a binarized representation to ease the judgement
    for seat in reserved_seats {
        if seat[1] == 1 || seat[1] == 10 { continue }
        if let Some(s) = map.get_mut(&seat[0]) {
            *s |= 1 << (seat[1] - 2);
        } else {
            map.insert(seat[0], 1 << (seat[1] - 2));
        }
    }

    // Here we cannot iterate all through the n as the time limit
    for (_, &binary) in map.iter() {
        if binary & 0xff == 0 { number += 2; }
        else if binary & 0xf == 0 || binary & 0x3c == 0 || binary & 0xf0 == 0 { 
            number += 1;
        }
    }
    number += 2 * (n - (map.len() as i32));
    number
}

#[test]
fn test() {
    let rv = vec![vec![1, 2], vec![1, 3], vec![1, 8], vec![2, 6], vec![3, 1], vec![3, 10]];
    assert_eq!(max_number_of_families(3, rv), 4);
}