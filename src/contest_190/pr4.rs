
fn max_dot_product(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let m = nums1.len();
    let n = nums2.len();
    let mut dp = vec![vec![0; n]; m];
    for i in 0..m {
        for j in 0..n {
            dp[i][j] = nums1[i] * nums2[j];
        }
    }
    let mut res = vec![vec![std::i32::MIN; n]; m];
    res[m-1][n-1] = dp[m-1][n-1];
    for i in (0..m).rev() {
        for j in (0..n).rev() {
            if i == m-1 && j == n-1 { continue; }
            else {
                let case0 = dp[i][j];
                let case1 = dp[i][j] + if i + 1 < m && j + 1 < n { res[i+1][j+1] } else { 0 };
                let case2 = if i + 1 < m { res[i+1][j] } else { std::i32::MIN }.max(if j + 1 < n {
                    res[i][j+1]
                } else { std::i32::MIN });
                res[i][j] = case0.max(case1).max(case2);
            }
        }
    }
    res[0][0]
}
#[test]
fn max_dot_product_test() {
    let nums1 = vec![-1,-1];
    let nums2 = vec![1,1];
    println!("{}", max_dot_product(nums1, nums2));
}