use std::collections::VecDeque;

fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
    let mut map = vec![(Vec::new(), Vec::new()); n as usize];
    for connection in connections.iter() {
        let src = connection[0] as usize;
        let dst = connection[1] as usize;
        map[src].0.push(dst);
        map[dst].1.push(src);
    }
    let mut count = 0;
    let mut queue = VecDeque::new();
    let mut degrees = vec![-1; n as usize];
    queue.push_back(0);
    degrees[0] = 0;
    while let Some(node) = queue.pop_front() {
        let degree = degrees[node];
        for to_node in map[node].0.iter() {
            if degrees[*to_node] == -1 {
                degrees[*to_node] = degree + 1;
                // reverse it
                count += 1;
                queue.push_back(*to_node);
            }
        }
        for from_node in map[node].1.iter() {
            if degrees[*from_node] == -1 {
                degrees[*from_node] = degree + 1;
                queue.push_back(*from_node);
            }
        }
    }    count
}
