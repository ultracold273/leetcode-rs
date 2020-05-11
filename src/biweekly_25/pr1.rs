

fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    let max = candies.iter().max().unwrap();
    let mut res = vec![false; candies.len()];
    for i in 0..candies.len() {
        if candies[i] + extra_candies >= *max { 
            res[i] = true; 
        }
    }
    res
}