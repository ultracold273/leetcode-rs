
fn backtrack(res: &mut Vec<Vec<i32>>, nums: &mut Vec<i32>, start: usize) {
    if start == nums.len() {
        res.push(nums.clone());
        return 
    }
    for i in start..nums.len() {
        let temp = nums[start];
        nums[start] = nums[i];
        nums[i] = temp;
        backtrack(res, nums, start + 1);
        let temp = nums[start];
        nums[start] = nums[i];
        nums[i] = temp;
    }
}

fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res = Vec::new();
    let mut nums = nums.clone();
    backtrack(&mut res, &mut nums, 0);
    res
}