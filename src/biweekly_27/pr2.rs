
fn has_all_codes(s: String, k: i32) -> bool {
    let k = k as usize;
    let mut appeared = vec![false; 1 << k];
    let n = s.len();
    let sv = s.chars().collect::<Vec<_>>();
    let mut cur_value = 0;
    for i in 0..n {
        cur_value = cur_value << 1;
        cur_value |= if sv[i] == '0' { 0 } else { 1 };
        if i >= k-1 {
            cur_value = cur_value & !(1 << k);
            appeared[cur_value] = true;
        }
    }
    appeared.iter().all(|&x| x)
}

#[test]
fn has_all_code_tests() {
    let s = "00110110".to_string();
    println!("{}", has_all_codes(s, 2));
}