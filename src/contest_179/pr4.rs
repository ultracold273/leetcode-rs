
fn dfs(start: i32, end: i32, graph: &Vec<Vec<i32>>, visited: &mut Vec<bool>, path: &mut Vec<i32>) -> bool {
    let mut res = false;
    visited[start as usize] = true;
    path.push(start);
    if start == end { res = true; }
    else {
        for cortex in &graph[start as usize] {
            if !visited[*cortex as usize] {
                res = dfs(*cortex, end, graph, visited, path);
                if res { break; }
            }
        }
    }
    visited[start as usize] = false;
    if !res { path.pop(); }
    res
}

fn frog_position(n: i32, edges: Vec<Vec<i32>>, t: i32, target: i32) -> f64 {
    let mut graph = vec![Vec::new(); n as usize + 1];
    for edge in edges {
        graph[edge[0] as usize].push(edge[1]);
        graph[edge[1] as usize].push(edge[0]);
    }
    let mut path = Vec::new();
    let mut visited = vec![false; n as usize + 1];
    dfs(1, target, &graph, &mut visited, &mut path);

    if t < (path.len() - 1) as i32 { 0f64 }
    else if t > (path.len() - 1) as i32 && graph[target as usize].len() > 1 { 0f64 }
    else if target == 1 && t > 0 && graph[1].len() > 0 { 0f64 }
    else {
        let mut prob = 1f64;
        for (i, node) in path.iter().take(path.len() - 1).enumerate() {
            if i == 0 {
                prob *= 1.0 / (graph[*node as usize].len() as f64);
            } else {
                prob *= 1.0 / (graph[*node as usize].len() as f64 - 1.0);
            }
        }
        prob
    }
}

#[test]
fn frog_position_test() {
    let n = 7;
    let edges = vec![vec![1,2],vec![1,3],vec![1,7],vec![2,4],vec![2,6],vec![3,5]];
    frog_position(n, edges, 2, 4);
}