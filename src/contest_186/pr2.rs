
fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
    let k = k as usize;
    let n = card_points.len();
    if k >= n {
        card_points.iter().sum()
    } else {
        // consecutive m elements not taken, find its minimum
        let m = n - k; 
        let total = card_points.iter().sum::<i32>();
        let mut dp = Vec::with_capacity(m);
        dp.push(card_points.iter().take(m).sum());
        for i in 1..=k {
            let value_i = dp[i-1] + card_points[i+m-1] - card_points[i-1];
            dp.push(value_i);
        }
        total - dp.iter().min().unwrap()
    }
}

#[test]
fn max_score_test() {
    assert_eq!(max_score(vec![1,2,3,4,5,6,1], 3), 12);
    assert_eq!(max_score(vec![100,40,17,9,73,75], 3), 248);
}