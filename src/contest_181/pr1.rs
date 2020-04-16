

pub fn create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut target = Vec::new();
    for i in 0..n {
        target.insert(index[i] as usize, nums[i]);
    }
    target
}

#[test]
fn create_target_array_test() {
    let nums = vec![0, 1, 2, 3, 4];
    let index = vec![0, 1, 2, 2, 1];
    assert_eq!(create_target_array(nums, index), vec![0, 4, 1, 3, 2]);
}
