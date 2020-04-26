
fn max_score(s: String) -> i32 {
    let mut zeros = vec![0; s.len()];
    let mut ones = vec![0; s.len()];
    for (i, c) in s.chars().enumerate() {
        if c == '0' {
            zeros[i] = if i == 0 {1} else { zeros[i-1] + 1 };
            ones[i] = if i == 0 {0} else {ones[i-1]};
        } else if c == '1' {
            zeros[i] = if i == 0 {0} else { zeros[i-1] };
            ones[i] = if i == 0 {1} else { ones[i-1] + 1 };
        }
    }
    (0..s.len()-1).map(|i| zeros[i] + ones[s.len()-1] - ones[i]).max().unwrap()
}