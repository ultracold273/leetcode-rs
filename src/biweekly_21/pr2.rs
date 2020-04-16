
fn find_the_longest_substring(s: String) -> i32 {
    let sv = s.chars().collect::<Vec<_>>();
    let mut res = 0usize;
    let mut status = vec![None; 32];
    status[0] = Some(-1);
    let mut longest = 0;
    for (i, c) in sv.iter().enumerate() {
        match *c {
            'a' => { res ^= 1 << 0; },
            'e' => { res ^= 1 << 1; },
            'i' => { res ^= 1 << 2; },
            'o' => { res ^= 1 << 3; },
            'u' => { res ^= 1 << 4; },
            _ => {},
        }
        if status[res].is_none() { status[res] = Some(i as i32); }
        else {
            let prev = status[res].unwrap();
            longest = longest.max(i as i32 - prev);
        }
    }
    longest
}