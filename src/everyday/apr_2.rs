
fn game_of_life(board: &mut Vec<Vec<i32>>) {
    let mut die = Vec::new();
    let mut revive = Vec::new();
    let m = board.len();
    let n = board[0].len();
    for i in 0..m {
        for j in 0..n {
            let mut alive_count = 0;
            alive_count += if i > 0 && j > 0 && board[i-1][j-1] == 1 { 1 } else { 0 };
            alive_count += if i > 0 && board[i-1][j] == 1 { 1 } else { 0 };
            alive_count += if i > 0 && j < n-1 && board[i-1][j+1] == 1 { 1 } else { 0 };
            alive_count += if j > 0 && board[i][j-1] == 1 { 1 } else { 0 };
            alive_count += if j < n-1 && board[i][j+1] == 1 { 1 } else { 0 };
            alive_count += if i < m-1 && j > 0 && board[i+1][j-1] == 1 { 1 } else { 0 };
            alive_count += if i < m-1 && board[i+1][j] == 1 { 1 } else { 0 };
            alive_count += if i < m-1 && j < n-1 && board[i+1][j+1] == 1 { 1 } else { 0 };
            if board[i][j] == 1 {
                if alive_count < 2 || alive_count > 3 { die.push((i, j)); }
            } else {
                if alive_count == 3 { revive.push((i, j)); }
            }
        }
    }
    
    die.iter().for_each(|cord| board[cord.0][cord.1] = 0);
    revive.iter().for_each(|cord| board[cord.0][cord.1] = 1);
}