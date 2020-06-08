

fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
    let nrow = grid.len();
    let ncol = grid[0].len();
    let mut dp = vec![vec![-1; ncol * ncol]; nrow];
    dp[0][0 * ncol + ncol - 1] = if ncol - 1 == 0 {
        grid[0][0]
    } else {
        grid[0][0] + grid[0][ncol - 1]
    };
    dp[0][(ncol - 1) * ncol] = dp[0][0 * ncol + ncol - 1];
    for i in 1..nrow {
        for j in 0..(ncol * ncol) {
            let p1 = j / ncol;
            let p2 = j % ncol;
            let mut prev = Vec::new();
            let offsets = vec![-1, 0, 1];
            for off1 in offsets.iter() {
                for off2 in offsets.iter() {
                    let np1 = p1 as i32 + off1;
                    let np2 = p2 as i32 + off2;
                    if np1 < 0 || np1 > ncol as i32 - 1 || np2 < 0 || np2 > ncol as i32 - 1 {
                        continue;
                    }
                    if dp[i - 1][np1 as usize * ncol + np2 as usize] != -1 {
                        prev.push(dp[i - 1][np1 as usize * ncol + np2 as usize]);
                    }
                }
            }
            if prev.len() > 0 {
                dp[i][j] = if p1 == p2 {
                    grid[i][p1]
                } else {
                    grid[i][p1] + grid[i][p2]
                };
                dp[i][j] += prev.iter().max().unwrap()
            }
        }
    }
    *dp[nrow - 1].iter().max().unwrap()
}
