use std::collections::binary_heap::BinaryHeap;
use std::cmp::Reverse;

fn max_performance(n: i32, speed: Vec<i32>, efficiency: Vec<i32>, k: i32) -> i32 {
    // Sort the struct in descending order
    let mut performance = speed.into_iter().map(|a| a as u64)
                               .zip(efficiency.into_iter().map(|a| a as u64))
                               .collect::<Vec<_>>();
    performance.sort_by(|a, b| b.1.cmp(&a.1));
    
    let mut min_heap = BinaryHeap::<Reverse<u64>>::new();
    let mut sum = 0u64;
    let mut max_perf = 0u64;
    for i in 0..k {
        let (spd, eff) = performance[i as usize];
        min_heap.push(Reverse(spd));
        sum += spd;
        max_perf = max_perf.max(eff * sum);
    }
    for i in k..n {
        let (spd, eff) = performance[i as usize];
        if spd > min_heap.peek().unwrap().0 {
            let old_min = min_heap.pop().unwrap().0;
            sum -= old_min;
            sum += spd;
            min_heap.push(Reverse(spd));
            max_perf = max_perf.max(eff * sum);
        }
    }
    (max_perf % (1000000000 + 7)) as i32
}

#[test]
fn max_performance_test() {
    assert_eq!(max_performance(6, vec![2,10,3,1,5,8], vec![56,4,3,9,7,2], 2), 60);
}