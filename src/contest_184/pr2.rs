
fn find(p: &Vec<i32>, q: i32) -> (usize, &i32) {
    p.iter().enumerate().find(|&(_, &c)| c == q).unwrap()
}

fn process_queries(queries: Vec<i32>, m: i32) -> Vec<i32> {
    let mut p = (1..=m).collect::<Vec<i32>>();
    let mut res = Vec::new();

    for q in queries {
        let (pos, &value) = find(&p, q);
        res.push(pos as i32);
        p.remove(pos);
        p.insert(0, value);
    }
    res
}
