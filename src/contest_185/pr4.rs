
const MOD: u64 = 1000000000 + 7;

fn num_of_arrays(n: i32, m: i32, k: i32) -> i32 {
    let n = n as usize;
    let m = m as usize;
    let k = k as usize;
    let mut dp = vec![vec![vec![0u64; m+1]; n+1]; n+1];
    for i in 1..=n {
        for j in 1..=k {
            for s in 1..=m {
                if i == 1 && j == 1 { dp[i][j][s] = 1; }
                else {
                    let mut sum = (dp[i-1][j][s] * s as u64) % MOD;
                    sum = (j-1..s).fold(sum, |psum, t| (dp[i-1][j-1][t] + psum) % MOD);
                    dp[i][j][s] = sum;
                }
            }
        }
    }
    let res = dp[n][k].iter().skip(k).fold(0u64, |sum, v| (sum + v) % MOD);
    res as i32
}

#[test]
fn num_of_arrays_test() {
    num_of_arrays(2, 3, 1);
}