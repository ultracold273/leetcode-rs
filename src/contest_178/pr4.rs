use std::collections::{VecDeque, HashSet};

fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
    let dir: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let (m, n) = (grid.len(), grid[0].len());
    let mut cost = vec![vec![-1; n]; m];
    let mut deque = VecDeque::new();
    let mut seen = HashSet::new();
    let valid = |x, y| x >= 0 && x < m as i32 && y >= 0 && y < n as i32;

    cost[0][0] = 0;
    deque.push_back((0, 0));
    while deque.len() > 0 {
        let (x, y) = deque.pop_front().unwrap();
        if seen.contains(&(x, y)) { continue; }
        else { seen.insert((x, y)); }
        for dir_idx in 0..4 {
            let (x_next, y_next) = (x + dir[dir_idx].0, y + dir[dir_idx].1);
            if valid(x_next, y_next) {
                let (x, y) = (x as usize, y as usize);
                let (x_next, y_next) = (x_next as usize, y_next as usize);
                if dir_idx == grid[x][y] as usize - 1 {
                    if cost[x_next][y_next] == -1 || cost[x_next][y_next] > cost[x][y] {
                        cost[x_next][y_next] = cost[x][y];
                        deque.push_front((x_next as i32, y_next as i32));
                    }
                } else {
                    if cost[x_next][y_next] == -1 || cost[x_next][y_next] > cost[x][y] + 1 {
                        cost[x_next][y_next] = cost[x][y] + 1;
                        deque.push_back((x_next as i32, y_next as i32));
                    }

                }
            }
        }
    }
    cost[m-1][n-1]
}