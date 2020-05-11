
use std::collections::HashMap;

fn count_triplets(arr: Vec<i32>) -> i32 {
    let n = arr.len();
    let mut dp = vec![0; n+1];
    for i in 0..n {
        dp[i+1] = dp[i] ^ arr[i];
    }
    let mut map = HashMap::new();
    let mut res = 0;
    for i in 0..n+1 {
        let list = map.entry(dp[i]).or_insert(Vec::new());
        if list.len() != 0 {
            for ii in list.iter() {
                res += (i - *ii - 1) as i32;
            }
        }
        list.push(i);
    }
    res
}