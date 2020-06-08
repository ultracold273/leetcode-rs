
// Dynamic Programming
// dp[i][j][k] - cost
// i - currently painted house
// j - blocks have been formed
// k - the color painted on i-th house
// Transfer:
// dp[i][j][k], iterate over cost[k][i+1] to update dp[i+1][j/j+1][t]

fn min_cost(houses: Vec<i32>, cost: Vec<Vec<i32>>, m: i32, n: i32, target: i32) -> i32 {
    let m = m as usize;
    let n = n as usize;
    let target = target as usize;
    let mut dp = vec![vec![vec![-1; n]; m]; m];
    // Only unpainted house can be painted
    if houses[0] == 0 {
        for color in 1..=n {
            if color == houses[0] as usize {
                dp[0][0][color - 1] = 0;
            } else {
                dp[0][0][color - 1] = cost[0][color - 1];
            }
        }
    } else {
        dp[0][0][houses[0] as usize - 1] = 0;
    }

    for i in 0..m - 1 {
        for j in 0..=i {
            for k in 0..n {
                if dp[i][j][k] == -1 {
                    continue;
                }

                if houses[i + 1] as usize != 0 {
                    let color = houses[i + 1] as usize;
                    let nblocks = if color - 1 == k { j } else { j + 1 };
                    if dp[i + 1][nblocks][color - 1] == -1 {
                        dp[i + 1][nblocks][color - 1] = dp[i][j][k];
                    } else {
                        dp[i + 1][nblocks][color - 1] = dp[i + 1][nblocks][color - 1].min(dp[i][j][k]);
                    }
                    continue;
                }

                for new_color in 1..=n {
                    let paint_cost = if houses[i + 1] as usize == new_color {
                        0
                    } else {
                        cost[i + 1][new_color - 1]
                    };
                    let nblocks = if new_color - 1 == k { j } else { j + 1 };
                    if dp[i + 1][nblocks][new_color - 1] == -1 {
                        dp[i + 1][nblocks][new_color - 1] = dp[i][j][k] + paint_cost;
                    } else {
                        dp[i + 1][nblocks][new_color - 1] =
                            dp[i + 1][nblocks][new_color - 1].min(dp[i][j][k] + paint_cost);
                    }
                }
            }
        }
    }

    let mut mcost = -1;
    for c in dp[m - 1][target - 1].iter() {
        if *c == -1 {
            continue;
        } else if mcost == -1 {
            mcost = *c;
        } else {
            mcost = mcost.min(*c);
        }
    }
    mcost
}

#[test]
fn min_cost_test() {
    let houses = vec![0,2,1,2,0];
    let cost = vec![vec![1, 10], vec![10, 1], vec![10, 1], vec![1, 10], vec![5, 1]];
    println!("{}", min_cost(houses, cost, 5, 2, 3));
}