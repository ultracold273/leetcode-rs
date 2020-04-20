
fn reformat(s: String) -> String {
    let mut letters = Vec::new();
    let mut digits = Vec::new();
    for &c in s.as_bytes() {
        if c >= '0' as u8 && c <= '9' as u8 {
            digits.push(c);
        } else if c >= 'a' as u8 && c <= 'z' as u8 {
            letters.push(c);
        }
    }
    let ln = letters.len();
    let dn = digits.len();
    if ln > dn + 2 || dn > ln + 2 {
        "".to_string()
    } else {
        let mut res = Vec::new();
        let mut count = 0;
        if ln > dn {
            while res.len() < s.len() { 
                if count % 2 == 0 { res.push(letters[count / 2]); }
                else {res.push(digits[count / 2]);}
            }
        } else {
            let mut count = 0;
            while res.len() < s.len() { 
                if count % 2 == 0 { res.push(digits[count / 2]); }
                else {res.push(letters[count / 2]);}
            }
        }
        count += 1;
        String::from_utf8(res).unwrap()
    }
}