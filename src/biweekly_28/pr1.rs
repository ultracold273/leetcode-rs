
fn final_prices(prices: Vec<i32>) -> Vec<i32> {
    let mut res = Vec::new();
    let n = prices.len();
    for i in 0..n {
        res.push(prices[i]);
        for j in i+1..n {
            if prices[j] <= prices[i] {
                res[i] = prices[i] - prices[j];
                break;
            }
        }
    }
    res
}