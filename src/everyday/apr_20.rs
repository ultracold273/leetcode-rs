
fn island_dfs(i: usize, j: usize, grid: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>) -> i32 {
    if grid[i][j] == '0' || visited[i][j] { return 0; }
    visited[i][j] = true;
    let m = grid.len();
    let n = grid[0].len();
    if i > 0 { island_dfs(i-1, j, grid, visited); }
    if i < m - 1 { island_dfs(i+1, j, grid, visited); }
    if j > 0 { island_dfs(i, j-1, grid, visited); }
    if j < n - 1 { island_dfs(i, j+1, grid, visited); }
    return 1;
}

fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    let m = grid.len();
    if m == 0 {
        return 0;
    }
    let n = grid[0].len();
    let mut visited = vec![vec![false; n]; m];
    let mut count = 0;
    for i in 0..m {
        for j in 0..n {
            count += island_dfs(i, j, &grid, &mut visited);
        }
    }
    count
}
