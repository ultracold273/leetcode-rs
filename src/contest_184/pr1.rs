
fn string_matching(words: Vec<String>) -> Vec<String> {
    let mut words = words.clone();
    words.sort_by(|a, b| a.len().cmp(&(b.len())));

    let n = words.len();
    let mut res = Vec::new();
    for i in 0..n {
        for j in i+1..n {
            if words[j].contains(&words[i]) {
                res.push(words[i].clone());
                break;
            }
        }
    }
    res
}