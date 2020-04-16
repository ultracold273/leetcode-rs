
fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
    let mut res = vec![0; nums.len()];
    let mut snums = nums.iter().enumerate().collect::<Vec<_>>();
    snums.sort_by(|a, b| a.1.cmp(b.1));
    let mut number = 0;
    let mut previous = None;
    for (idx, &tuple) in snums.iter().enumerate() {
        let (res_idx, &num) = tuple;
        if let Some(prev) = previous {
            if prev != num {
                number = idx as i32;
                previous = Some(num);
            }
        } else {
            previous = Some(num);
        }
        res[res_idx] = number;
    }
    res
}