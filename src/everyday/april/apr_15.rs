fn update_matrix(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let m = matrix.len();
    let n = matrix[0].len();
    let mut res = vec![vec![0; n]; m];
    
    for i in 0..m {
        for j in 0..n {
            if matrix[i][j] != 0 {
                let up = if i > 0 { res[i-1][j] } else { 10000  };
                let left = if j > 0 { res[i][j-1] } else { 10000 };
                res[i][j] = up.min(left) + 1;
            } else {
                res[i][j] = 0;
            }
        }
    }
    
    for i in (0..m).rev() {
        for j in (0..n).rev() {
            if matrix[i][j] != 0 {
                let bottom = if i < m - 1 { res[i+1][j] } else { 10000 };
                let right = if j < n - 1 { res[i][j+1] } else { 10000 };
                res[i][j] = (bottom.min(right) + 1).min(res[i][j]);
            } else {
                res[i][j] = 0;
            }
        }
    }
    res
}