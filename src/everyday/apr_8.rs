
fn digit_sum(x: i32) -> i32 {
    let mut sum = 0;
    let mut x = x;
    while x > 0 { sum += x % 10; x /= 10; }
    sum
}

fn legal(x: i32, y: i32, k: i32, visited: &Vec<Vec<bool>>) -> bool {
    let m = visited.len();
    let n = visited[0].len();
    if x < 0 || x > (m as i32 - 1) || y < 0 || y > (n as i32 - 1) {
        false
    } else if visited[x as usize][y as usize] {
        false
    } else if digit_sum(x) + digit_sum(y) > k {
        false
    } else {
        true
    }
}

fn dfs(x: i32, y: i32, k: i32, visited: &mut Vec<Vec<bool>>) -> i32 {
    let mut res = 0;
    if legal(x, y, k, visited) {
        visited[x as usize][y as usize] = true;
        res += 1;
        res += dfs(x+1, y, k, visited);
        res += dfs(x-1, y, k, visited);
        res += dfs(x, y+1, k, visited);
        res += dfs(x, y-1, k, visited);
    }
    res
}

fn moving_count(m: i32, n: i32, k: i32) -> i32 {
    let mut visited = vec![vec![false; n as usize]; m as usize];
    dfs(0, 0, k, &mut visited)
}

#[test]
fn moving_count_test() {
    println!("{}", moving_count(1, 2, 1));
}