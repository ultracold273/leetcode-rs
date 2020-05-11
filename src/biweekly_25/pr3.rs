fn check_if_can_break(s1: String, s2: String) -> bool {
    let mut s1v = s1.chars().collect::<Vec<_>>();
    let mut s2v = s2.chars().collect::<Vec<_>>();
    s1v.sort(); s2v.sort();
    let n = s1.len();
    let mut res = true;
    for i in 0..n {
        if s1v[i] >= s2v[i] {} else { res = false; break; }
    }
    if res { return res; }
    res = true;
    for i in 0..n {
        if s2v[i] >= s1v[i] {} else { res = false; break; }
    }
    return res
}

#[test]
fn check_if_can_break_test() {
    println!("{}", check_if_can_break("abe".to_string(), "acd".to_string()));
}