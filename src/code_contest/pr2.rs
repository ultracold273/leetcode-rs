
use std::collections::HashMap;

fn dfs(start: i32, end: i32, cur: i32, k: i32, edges: &HashMap<i32, Vec<i32>>) -> i32 {
    if cur == k {
        if start == end { 1 } else { 0 }
    } else {
        if let Some(next) = edges.get(&start) {
            let mut sum = 0;
            for e in next {
                sum += dfs(*e, end, cur+1, k, edges);
            }
            sum
        } else {
            0
        }
    }
}

fn num_ways(n: i32, relation: Vec<Vec<i32>>, k: i32) -> i32 {
    let mut edges = HashMap::<i32, Vec<i32>>::new();
    relation.iter().for_each(
        |a| edges.entry(a[0]).or_insert(Vec::new()).push(a[1])
    );

    dfs(0, n - 1, 0, k, &edges)
}