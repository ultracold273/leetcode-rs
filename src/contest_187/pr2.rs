
fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
    let mut start = 0;
    while start < nums.len() && nums[start] != 1 { start += 1; }
    if start == nums.len() { return true; }
    let mut last1 = start;
    for i in start+1..nums.len() {
        if nums[i] == 1 {
            if i - last1 <= k as usize { return false; }
            else { last1 = i; }
        }
    }
    true
}