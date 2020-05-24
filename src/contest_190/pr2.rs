// use std::collections::VecDeque;

fn max_vowels(s: String, k: i32) -> i32 {
    // let mut queue = VecDeque::new();
    let mut nmax = 0;
    let mut cur_vowels = 0;
    for (i, c) in s.chars().enumerate() {
        if i < k as usize {
            if c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' {
                cur_vowels += 1;
            }
            // queue.push_back(c);
        } else {
            if c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' {
                cur_vowels += 1;
            }
            // queue.push_back(c);
            // let c = queue.pop_front().unwrap();
            let c = s.chars().nth(i - k as usize).unwrap();
            if c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' {
                cur_vowels -= 1;
            }
        }
        nmax = nmax.max(cur_vowels);
    }
    nmax
}