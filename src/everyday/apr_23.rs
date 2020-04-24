
const MOD: u64 = 1000000000 + 7;

fn ways_to_change(n: i32) -> i32 {
    let n = n as usize;
    let mut dp = vec![0u64; n as usize + 1];
    let coins = vec![1, 5, 10, 25];
    dp[0] = 1;
    for c in coins {
        for j in c..=n {
            dp[j] = (dp[j] + dp[j - c]) % MOD;
        }
    }
    dp[n] as i32
}