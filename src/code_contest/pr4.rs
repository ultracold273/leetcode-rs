
// DP
fn min_jump(jump: Vec<i32>) -> i32 {
    let mut dp = vec![1000000; jump.len()];
    let mut max_idx = vec![0; jump.len()];
    dp[0] = 0; max_idx[0] = 0; 
    let mut k = 0; // jump times
    let mut res = 100000;
    for i in 0..jump.len() {
        if i > max_idx[k] {
            k += 1;
        }
        dp[i] = dp[i].min(k + 1);
        let next = i + jump[i] as usize;
        if next >= jump.len() { res = res.min(dp[i] + 1); continue; }
        dp[next] = dp[next].min(dp[i] + 1);
        max_idx[dp[next]] = max_idx[dp[next]].max(next);
    }
    res as i32
}

fn min_jump_bfs(jump: Vec<i32>) -> i32 {
    0
}

#[test]
fn min_jump_test() {
    println!("{}", min_jump(vec![3,7,6,1,4,3,7,8,1,2,8,5,9,8,3,2,7,5,1,1]));
}