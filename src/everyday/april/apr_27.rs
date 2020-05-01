
fn search(nums: Vec<i32>, target: i32) -> i32 {
    if nums.len() == 0 { return -1; }
    let mut lo = 0;
    let mut hi = nums.len() - 1;
    while lo + 1 < hi {
        let mid = (lo + hi) / 2;
        if nums[mid] > nums[lo] {
            if nums[mid] > target && nums[lo] <= target { hi = mid; }
            else { lo = mid; }
        } else {
            if nums[mid] <= target && nums[hi] >= target { lo = mid; }
            else { hi = mid; }
        }
    }
    if target == nums[lo] { lo as i32 }
    else if target == nums[hi] { hi as i32 }
    else { -1 }
}