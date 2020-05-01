 fn rotate(matrix: &mut Vec<Vec<i32>>) {
    let n = matrix.len();
    for i in 0..(n+1)/2 {
        for j in i..n-i-1 {
            let temp = matrix[i][j];
            matrix[i][j] = matrix[n-1-j][i];
            matrix[n-1-j][i] = matrix[n-1-i][n-1-j];
            matrix[n-1-i][n-1-j] = matrix[j][n-1-i];
            matrix[j][n-1-i] = temp;
        }
    }
}
