

fn min_start_value(nums: Vec<i32>) -> i32 {
    let mut min = std::i32::MAX;
    let mut sum = 0;
    for num in nums {
        sum += num;
        min = min.min(sum);
    }
    if min > 0 { 1 } else { 1 - min }
}