
use std::collections::BinaryHeap;

fn kth_smallest(mat: Vec<Vec<i32>>, k: i32) -> i32 {
    let m = mat.len();
    let n = mat[0].len();
    let mut left = mat.iter().fold(0, |sum, list| sum + list[0]);
    let mut right = mat.iter().fold(0, |sum, list| sum + list[n-1]);
    let init = left;
    while left < right {
        let mid = (left + right) / 2;
        let count = count_leq_than_mid(&mat, mid, 0, init, k);
        if count < k {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    right
}

fn count_leq_than_mid(mat: &Vec<Vec<i32>>, mid: i32, index: usize, sum: i32, k: i32) -> i32 {
    if index == mat.len() { 1 }
    else {
        let mut count = 0;
        let n = mat[0].len();
        for i in 0..n {
            let new_sum = sum + mat[index][i] - mat[index][0];
            if new_sum <= mid {
                count += count_leq_than_mid(mat, mid, index+1, new_sum, k);
                if count >= k { return count; }
            } else {
                break;
            }
        }
        count
    }
}

#[test]
fn kth_smallest_test() {
    let mat = vec![vec![1,3,11],vec![2,4,6]];
    println!("{}", kth_smallest(mat, 5));
}