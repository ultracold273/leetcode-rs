
// TLE see explanation, one k should be enough
use std::collections::BinaryHeap;

fn is_possible(target: Vec<i32>) -> bool {
    let mut heap = BinaryHeap::new();
    let mut sum = target.iter().map(|a| *a as u64).sum::<u64>();
    target.iter().for_each(|t| heap.push(*t));

    while let Some(max) = heap.pop() {
        let max64 = max as u64;
        if max64 == 1 { break; }
        if max64 < sum - max64 { return false; }
        let prev = max64 - (sum - max64);
        heap.push(prev as i32);
        sum = sum - max64 + prev;
    }
    true
}