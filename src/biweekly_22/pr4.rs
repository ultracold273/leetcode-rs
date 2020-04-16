
// 1. simplify the ring to a flat array
// 2. sub-array maximum sum problem

fn max_size(slices: &[i32], k: usize) -> i32 {
    let n = slices.len();
    let mut dp = vec![vec![0; n]; k];
    for i in 0..k {
        for j in 0..n {
            if i == 0 { dp[i][j] = slices[j]; }
            else if j < i { dp[i][j] = 0; }
            else if let Some(max) = dp[i-1][0..j-1].iter().max() {
                dp[i][j] = slices[j] + max;
            }
        }
    }
    
    *dp[k-1][0..n].iter().max().unwrap()
}

fn max_size_slices(slices: Vec<i32>) -> i32 {
    let n = slices.len();
    let m1 = max_size(&slices[0..n-1], n / 3);
    let m2 = max_size(&slices[1..n], n / 3);
    m1.max(m2)
}

#[test]
fn max_size_slices_test() {
    let slice = vec![1, 2, 3, 4, 5, 6];
    assert_eq!(max_size_slices(slice), 10);
}