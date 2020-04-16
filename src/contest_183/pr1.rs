
fn min_subsequence(nums: Vec<i32>) -> Vec<i32> {
    let mut mnums = nums.clone();
    let sum = mnums.iter().sum();
    mnums.sort_by(|&a, &b| b.cmp(&a));

    let mut accumulate = 0;
    let mut res = Vec::new();
    for n in mnums {
        accumulate += n;
        res.push(n);
        if 2 * accumulate > sum { break; }
    }
    res
}