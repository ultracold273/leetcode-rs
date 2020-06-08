
fn max_product(nums: Vec<i32>) -> i32 {
    let mut max_prod = 0;
    for i in 0..nums.len() {
        for j in 0..i {
            let prod = (nums[i] - 1) * (nums[j] - 1);
            max_prod = max_prod.max(prod);
        }
    }
    max_prod
}