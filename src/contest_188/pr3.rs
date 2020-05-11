
fn traverse(root: usize, tree: &Vec<Vec<usize>>, has_apple: &Vec<bool>, children_apple: &mut Vec<bool>) -> bool {
    let mut res = if has_apple[root] { true } else { false };
    for child in tree[root].iter() {
        let r = traverse(*child, tree, has_apple, children_apple);
        children_apple[*child] = r;
        res = res || r;
    }
    res
}

fn traverse_time(root: usize, tree: &Vec<Vec<usize>>, children_apple: &Vec<bool>) -> i32 {
    let mut res = 0;
    if children_apple[root] {
        res += 2;
        for child in tree[root].iter() {
            res += traverse_time(*child, tree, children_apple);
        }
    }
    res
}

fn min_time(n: i32, edges: Vec<Vec<i32>>, has_apple: Vec<bool>) -> i32 {
    let n = n as usize;
    let mut children_has_apple = vec![false; n];
    let mut tree = vec![Vec::new(); n];
    for edge in edges.iter() {
        let parent = edge[0] as usize;
        let child = edge[1] as usize;
        tree[parent].push(child);
    }
    // println!("{:?}", tree);
    let root_has_apple = traverse(0, &tree, &has_apple, &mut children_has_apple);
    // children_has_apple[0] = root_has_apple;
    println!("{:?}", children_has_apple);
    if root_has_apple {
        traverse_time(0, &tree, &mut children_has_apple) - 2
    } else { 0 }
}

#[test]
fn min_time_test() {
    // let n = 7; 
    // let edges = vec![vec![0,1],vec![0,2],vec![1,4],vec![1,5],vec![2,3],vec![2,6]];
    // let hasApple = vec![false,false,true,false,true,true,false];
    
    let n = 4;
    let edges = vec![vec![0,1],vec![1,2],vec![0,3]];
    let hasApple = vec![true,true,true,true];
    println!("{}", min_time(n, edges, hasApple));
}