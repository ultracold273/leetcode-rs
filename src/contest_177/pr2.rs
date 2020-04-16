fn traverse(n: usize, l: &Vec<i32>, r: &Vec<i32>, v: &mut Vec<bool>) -> bool {
    if v[n] { false }
    else {
        v[n] = true;
        let left = if l[n] == -1 { true } else { traverse(l[n] as usize, l, r, v) };
        let right = if r[n] == -1 { true } else { traverse(r[n] as usize, l, r, v) };
        left && right
    }
}

fn find_root(n: usize, l: &Vec<i32>, r: &Vec<i32>) -> Option<usize> {
    let mut count = vec![0; n];
    l.iter().for_each(|&x| if x != -1 {count[x as usize] += 1;});
    r.iter().for_each(|&x| if x != -1 {count[x as usize] += 1;});

    let mut zero_cnt = 0;
    let mut zero_index = 0;
    for (i, &c) in count.iter().enumerate() {
        if c != 0 && c != 1 {
            return None;
        }
        if c == 0 { zero_cnt += 1; zero_index = i; }
    }
    if zero_cnt != 1 { None } else { Some(zero_index) }
}

fn validate_binary_tree_nodes(n: i32, left_child: Vec<i32>, right_child: Vec<i32>) -> bool {
    if let Some(root) = find_root(n as usize, &left_child, &right_child) {
        let mut visited = vec![false; n as usize];
        traverse(root, &left_child, &right_child, &mut visited) && visited.iter().fold(true, |a, &b| a && b)
    } else {
        false
    }
}