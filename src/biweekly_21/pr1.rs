fn sort_string(s: String) -> String {
    let mut ccount = vec![0; 26];
    for c in s.chars() {
        let ci = c as u8 - 'a' as u8;
        ccount[ci as usize] += 1;
    }

    let mut res = Vec::new();
    let mut up = true;
    while res.len() < s.len() {
        if up {
            for (ci, count) in ccount.iter_mut().enumerate() {
                if *count > 0 {
                    res.push((ci as u8 + 'a' as u8) as char);
                    *count -= 1;
                }
            }
        } else {
            for (ci, count) in ccount.iter_mut().enumerate().rev() {
                if *count > 0 {
                    res.push((ci as u8 + 'a' as u8) as char);
                    *count -= 1;
                }
            }
        }
        up = !up;
    }
    res.iter().collect()
}

#[test]
fn sort_string_test() {
    let s = "aaaabbbbcccc".to_string();
    println!("{}", sort_string(s));
}