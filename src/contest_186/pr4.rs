
// dynamic programming
// transfer function:
// dp[i] = max(dp[i-j] + nums[i], nums[i]) for j = 1, ..., k --> TLE
// k maybe large, iterating all k TLE
// use monotonuous queue to store the [dp[i-k]..dp[i-1]]
use std::collections::VecDeque;
fn constrained_subset_sum(nums: Vec<i32>, k: i32) -> i32 {
    let k = k as usize;
    let n = nums.len();
    let mut dp = vec![0; n];
    let mut queue = VecDeque::new();
    let mut max = nums[0];
    queue.push_back(0);
    for i in 0..n {
        if i == 0 { dp[i] = nums[i]; continue; }
        else {
            let max_idx = *(queue.front().unwrap());
            dp[i] = nums[i].max(dp[max_idx] + nums[i]);
            if i == max_idx + k { queue.pop_front(); }
            println!("{} {} {} {}", i, max_idx, max, dp[i]);
            max = max.max(dp[i]);
            // println!("{} {} {}", i, max_idx, max);
        }
        while !queue.is_empty() && dp[i] >= dp[*queue.back().unwrap()] { 
            queue.pop_back(); 
        }
        queue.push_back(i);
    }
    max
}

#[test]
fn constrained_subset_sum_test() {
    // println!("{}", constrained_subset_sum(vec![10,2,-10,5,20], 2));
    println!("{}", constrained_subset_sum(vec![-1, -2, -3], 1));
}