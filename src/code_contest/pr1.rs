
fn min_count(coins: Vec<i32>) -> i32 {
    let mut count = Vec::new();
    for coin in coins {
        let c = (coin + 1) / 2;
        count.push(c);
    }
    count.iter().sum()
}