
fn can_jump(nums: Vec<i32>) -> bool {
    let n = nums.len();
    let mut dp = vec![false; n];
    dp[n-1] = true;
    for i in (0..n-1).rev() {
        let mlen = nums[i] as usize;
        if mlen == 0 { dp[i] = false; }
        else {
            let end = if i + mlen < n { i + mlen } else { n - 1 };
            dp[i] = dp[i+1..=end].iter().any(|a| *a);
        }
    }
    dp[0]
}