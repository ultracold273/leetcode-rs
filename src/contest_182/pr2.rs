
fn num_teams_direction(rating: &Vec<i32>, up: bool, n: usize) -> i32 {
    let m = rating.len();
    let mut dp = vec![vec![0; m]; n];
    let mut nsum = 0;

    for j in 0..n {
        for i in 0..m {
            if j == 0 { dp[j][i] = 1; }
            else if i < j { dp[j][i] = 0 }
            else {
                let mut num = 0;
                for t in 0..i {
                    if up && rating[t] < rating[i] { num += dp[j-1][t]; }
                    if !up && rating[t] > rating[i] { num += dp[j-1][t]; }
                }
                dp[j][i] = num;
                if j == m-1 { nsum += num; }
            }
        }
    }
    nsum
}

fn num_teams(rating: Vec<i32>) -> i32 {
    num_teams_direction(&rating, true, 3) + num_teams_direction(&rating, false, 3)
}

#[test]
fn num_teams_test() {
    let tcases = vec![(vec![2, 5, 3, 4, 1], 3), (vec![2, 1, 3], 0), (vec![1, 2, 3, 4], 4)];
    for (t, r) in tcases {
        assert_eq!(num_teams(t), r);
    }
}