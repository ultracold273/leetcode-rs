
fn min_steps(s: String, t: String) -> i32 {
    let mut count = vec![0; 26];
    for c in s.chars() {
        let index = c as u8 - 'a' as u8;
        count[index as usize] += 1;
    }

    for c in t.chars() {
        let index = c as u8 - 'a' as u8;
        if count[index as usize] > 0 {
            count[index as usize] -= 1;
        }
    }
    count.iter().sum()
}