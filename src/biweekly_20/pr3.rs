fn number_of_substrings(s: String) -> i32 {
    let sv = s.chars().collect::<Vec<_>>();
    let mut dp = vec![vec![0; 7]; s.len()];
    match sv[0] {
        'a' => {dp[0][0] = 1;},
        'b' => {dp[0][1] = 1;},
        'c' => {dp[0][2] = 1;},
        _ => {}
    }

    for i in 1..s.len() {
        match sv[i] {
            'a' => { 
                dp[i][0] = dp[i-1][0] + 1; dp[i][1] = 0; dp[i][2] = 0;
                dp[i][3] = dp[i-1][3] + dp[i-1][1]; 
                dp[i][4] = dp[i-1][4] + dp[i-1][2];
                dp[i][5] = 0;
                dp[i][6] = dp[i-1][6] + dp[i-1][5];
            },
            'b' => {
                dp[i][0] = 0; dp[i][1] = dp[i-1][1] + 1; dp[i][2] = 0;
                dp[i][3] = dp[i-1][3] + dp[i-1][0]; 
                dp[i][4] = 0; 
                dp[i][5] = dp[i-1][5] + dp[i-1][2];
                dp[i][6] = dp[i-1][6] + dp[i-1][4];
            },
            'c' => {
                dp[i][0] = 0; dp[i][1] = 0; dp[i][2] = dp[i-1][2] + 1;
                dp[i][3] = 0; dp[i][4] = dp[i-1][4] + dp[i-1][0]; dp[i][5] = dp[i-1][5] + dp[i-1][1];
                dp[i][6] = dp[i-1][6] + dp[i-1][3];
            },
            _ => {},
        }
    }
    
    dp.iter().fold(0, |a, b| a + b[6])
}