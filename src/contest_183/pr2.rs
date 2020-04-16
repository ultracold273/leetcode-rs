
fn num_steps_rec(s: &mut Vec<char>, start: usize) -> i32 {
    let mut count = 0;
    let mut pos = start;
    if pos == s.len() - 1 && s[pos] == '1' {
        // count
    } else if s[pos] == '0'{
        while s[pos] == '0' { pos += 1; } 
        count = (pos - start) as i32;
        count += num_steps_rec(s, pos);
    } else {
        while pos < s.len() && s[pos] == '1' { pos += 1; }
        count = (pos - start + 1) as i32;
        if pos == s.len() {s.push('1');} else {s[pos] = '1';}
        count += num_steps_rec(s, pos);
    }
    count
}

fn num_steps(s: String) -> i32 {
    let mut cs = s.chars().rev().collect::<Vec<_>>();
    println!("{:?}", cs);
    num_steps_rec(&mut cs, 0)
}

#[test]
fn num_steps_test() {
    let s = "1101".to_string();
    assert_eq!(num_steps(s), 6);
}