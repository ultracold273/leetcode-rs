fn single_numbers(nums: Vec<i32>) -> Vec<i32> {
    let mut xor = nums.iter().fold(0, |a, b| a ^ b);
    let mut cnt = 0;
    while xor != 0 {
        if xor & 1 != 0 {
            break;
        }
        cnt += 1;
        xor = xor >> 1;
    }
    let mut arr0 = Vec::new();
    let mut arr1 = Vec::new();
    nums.iter().for_each(|x| {
        if x & (1 << cnt) != 0 {
            arr1.push(*x)
        } else {
            arr0.push(*x)
        }
    });
    vec![
        arr0.iter().fold(0, |a, b| a ^ b),
        arr1.iter().fold(0, |a, b| a ^ b),
    ]
}
