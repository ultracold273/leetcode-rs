
fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut res = 0;
    let mut col = n;
    for i in 0..m {
        for j in 0..col {
            if grid[i][j] < 0 {
                res += (col - j) * (m - i);
                col = j;
                break;
            }
        }
    }
    res as i32
}

#[test]
fn name() {
    count_negatives(vec![vec![4,3,2,-1],vec![3,2,1,-1],vec![1,1,-1,-2],vec![-1,-1,-2,-3]]);
}