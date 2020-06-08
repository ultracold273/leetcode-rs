use std::collections::{HashMap, VecDeque};

fn check(n: i32, tree: &HashMap<i32, Vec<i32>>, query: &Vec<i32>) -> bool {
    let pre_class = query[0];
    let mut classes = VecDeque::new();
    classes.push_back(query[1]);
    let mut visited = vec![false; n as usize];
    while let Some(class) = classes.pop_front() {
        if let Some(prerequisites) = tree.get(&class) {
            for pclass in prerequisites.iter() {
                let pclass = *pclass as usize;
                if visited[pclass] {
                    continue;
                } else {
                    visited[pclass] = true;
                    if pclass == pre_class as usize {
                        return true;
                    }
                    classes.push_back(pclass as i32);
                }
            }
        }
    }
    false
}

fn check_if_prerequisite(
    n: i32,
    prerequisites: Vec<Vec<i32>>,
    queries: Vec<Vec<i32>>,
) -> Vec<bool> {
    let mut tree = HashMap::new();
    for prerequisite in prerequisites.iter() {
        let pre_class = prerequisite[0];
        let class = prerequisite[1];
        let prerequisite_per_class = tree.entry(class).or_insert(Vec::new());
        prerequisite_per_class.push(pre_class);
    }
    let mut res = Vec::new();
    for query in queries.iter() {
        res.push(check(n, &tree, query));
    }
    res
}
#[test]
fn check_if_prerequisite_test() {
    // println!("{:?}", check_if_prerequisite(2, vec![vec![1, 0]], vec![vec![1,0], vec![0,1]]));
    let n = 3;
    let pres = vec![vec![1, 0], vec![2, 0]];
    let que = vec![vec![0, 1], vec![2, 0]];
    println!("{:?}", check_if_prerequisite(n, pres, que));
}
