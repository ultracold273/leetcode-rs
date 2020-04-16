
fn longest_prefix(s: String) -> String {
    let n = s.len();
    let sv = s.chars().collect::<Vec<_>>();
    let mut next = vec![-1i32; n + 1];
    let mut j = next[0];

    for i in 0..n {
        while j >= 0 {
            let jj = j as usize;
            if sv[i] == sv[jj] { break; }
            else { j = next[jj]; }
        }
        j += 1;
        next[i + 1] = j;
    }
    s[0..next[n] as usize].to_owned()
}
