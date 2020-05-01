
// interesting problem

fn dfs(k: usize, n: usize, dp: &mut Vec<Vec<i32>>) -> i32 {
    let mut ans = 0;
    if dp[n][k] != -1 {
        return dp[n][k];
    } else if k == 1 {
        ans = n as i32;
    } else if n == 0 {
        ans = 0;
    } else {
        let mut lo = 1;
        let mut hi = n;
        while lo + 1 < hi {
            let x = (lo + hi) / 2;
            let t1 = dfs(k - 1, x - 1, dp);
            let t2 = dfs(k, n - x, dp);

            if t1 < t2 { lo = x; }
            else if t1 > t2 { hi = x; }
            else { lo = x; hi = x; }
        }
        ans = 1 + dfs(k-1, lo-1, dp).max(dfs(k, n-lo, dp)).min(dfs(k-1, hi-1, dp).max(dfs(k, n-hi, dp)));
    }
    dp[n][k] = ans;
    ans
}

fn super_egg_drop(k: i32, n: i32) -> i32 {
    let mut dp = vec![vec![-1; (k+1) as usize]; (n+1) as usize];
    dfs(k as usize, n as usize, &mut dp)
}

#[test]
fn super_egg_drop_test() {
    println!("{}", super_egg_drop(1, 2));
    println!("{}", super_egg_drop(2, 6));
    println!("{}", super_egg_drop(3, 14));
}