
fn check_if_exist(arr: Vec<i32>) -> bool {
    let n = arr.len();
    for i in 0..n {
        for j in 0..n {
            if i == j { continue; }
            if arr[i] == 2 * arr[j] {
                return true;
            }
        }
    }
    false
}