
const MOD: u64 = 1000000000 + 7;

fn ways(pizza: Vec<String>, k: i32) -> i32 {
    let k = k as usize;
    let row = pizza.len();
    let col = pizza[0].len();
    let mut pizza_counts = vec![vec![0u64; col]; row];
    for (r, rp) in pizza.iter().rev().enumerate() {
        let mut line_apple = 0;
        for (c, apple) in rp.chars().rev().enumerate() {
            if apple == 'A' { line_apple += 1; }
            if r == 0 {
                pizza_counts[r][c] = line_apple;
            } else {
                pizza_counts[r][c] = pizza_counts[r-1][c] + line_apple;
            }
        }
    }
    // println!("{:?}", pizza_counts);
    let mut dp = vec![vec![vec![0u64; col]; row]; k+1];
    for i in 0..row {
        for j in 0..col {
            for kk in 0..=k {
                if i + j < kk { dp[kk][i][j] = 0; continue; }
                if kk == 0 {
                    dp[kk][i][j] = if pizza_counts[i][j] > 0 { 1 } else { 0 };
                } else {
                    for ii in 0..i {
                        if pizza_counts[i][j] - pizza_counts[i-1-ii][j] > 0 {
                            dp[kk][i][j] = (dp[kk][i][j] + dp[kk-1][i-1-ii][j]) % MOD;
                        }
                    }
                    for jj in 0..j {
                        if pizza_counts[i][j] - pizza_counts[i][j-1-jj] > 0 {
                            dp[kk][i][j] = (dp[kk][i][j] + dp[kk-1][i][j-1-jj]) % MOD;
                        }
                    }
                }
            }
        }
    }
    println!("{:?}", dp);
    dp[k-1][row-1][col-1] as i32
}

#[test]
fn ways_test() {
    let pizza1 = vec!["A..".to_string(), "AAA".to_string(), "...".to_string()];
    let k1 = 3;
    println!("{}", ways(pizza1, k1));
    
    let pizza2 = vec!["A..".to_string(),"AA.".to_string(),"...".to_string()];
    let k2 = 3;
    println!("{}", ways(pizza2, k2));
    
    let pizza3 = vec!["A..".to_string(),"A..".to_string(),"...".to_string()];
    let k3 = 1;
    println!("{}", ways(pizza3, k3));
    let pizza = vec![
        "AAAA.".to_string(),
        "A..A.".to_string(),
        "AA.AA".to_string(),
    ];
    let k = 5;
    println!("{}", ways(pizza, k));
}