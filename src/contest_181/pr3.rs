
pub fn valid_path(grid: &Vec<Vec<i32>>, idirection: i32) -> bool {
    let m = grid.len();
    let n = grid[0].len();
    let mut visited = vec![vec![false; n]; m];
    let mut i = 0;
    let mut j = 0;
    let mut direction = idirection;
    loop {
        visited[i][j] = true;
        if i == 0 && j == 0 {
            if grid[i][j] == 5 { return false; }
            else if grid[i][j] == 1 || grid[i][j] == 6 {
                j += 1;
                direction = 4;
            } else if grid[i][j] == 2 || grid[i][j] == 3 {
                i += 1;
                direction = 1;
            } else if direction == 1 {
                i += 1;
            } else {
                j += 1;
            }
        } else if i == m - 1 && j == n - 1 {
            if direction == 1 { return grid[i][j] == 2 || grid[i][j] == 6 || grid[i][j] == 5 }
            else if direction == 4 { return grid[i][j] == 1 || grid[i][j] == 3 || grid[i][j] == 5}
        } else {
            if direction == 1 {
                if grid[i][j] == 2 { i += 1; direction = 1; }
                else if grid[i][j] == 5 { j -= 1; direction = 3; }
                else if grid[i][j] == 6 { j += 1; direction = 4; }
                else { return false; } 
            } else if direction == 2 {
                if grid[i][j] == 2 { i -= 1; direction = 2; }
                else if grid[i][j] == 3 { j -= 1; direction = 3; }
                else if grid[i][j] == 4 { j += 1; direction = 4; }
                else { return false; }
            } else if direction == 3 {
                if grid[i][j] == 1 { j -= 1; direction = 3; }
                else if grid[i][j] == 4 { i += 1; direction = 1; }
                else if grid[i][j] == 6 { i -= 1; direction = 2; }
                else { return false; }
            } else if direction == 4 {
                if grid[i][j] == 1 { j += 1; direction = 4; }
                else if grid[i][j] == 3 { i += 1; direction = 1; }
                else if grid[i][j] == 5 { i -= 1; direction = 2; }
                else { return false; }
            }
        }

        if i >= m || j >= n { return false; }
        if visited[i][j] { return false; }
    }
}

fn has_valid_path(grid: Vec<Vec<i32>>) -> bool {
    valid_path(&grid, 1) || valid_path(&grid, 4)
}

#[test]
fn contest_181_pr3_test1() {
    // assert_eq!(2 + 2, 4);
    let grid = vec![vec![2, 4, 3], vec![6, 5, 2]];
    assert_eq!(has_valid_path(grid), true);
}

#[test]
fn contest_181_pr3_test2() {
    let grid = vec![vec![4, 1], vec![6, 1]];
    assert_eq!(has_valid_path(grid), true);
}

#[test]
fn contest_181_pr3_test3() {
    let grid = vec![vec![6, 1, 3], vec![4, 1, 5]];
    assert_eq!(has_valid_path(grid), true);
}

#[test]
fn contest_181_pr3_test4() {
    let grid = vec![vec![3, 4, 3, 4], vec![2, 2, 2, 2], vec![6, 5, 6, 5]];
    assert_eq!(has_valid_path(grid), true);
}