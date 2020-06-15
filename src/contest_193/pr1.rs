fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    let mut res = Vec::new();
    res.push(nums[0]);
    let n = nums.len();
    for i in 1..n {
        res.push(res[i-1] + nums[i]);
    }
    res
}