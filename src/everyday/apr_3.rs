
enum State {
    LEADING,
    DIGITS,
}

fn my_atoi(str: String) -> i32 {
    let mut state = State::LEADING;
    let mut sign = None;
    let mut res = 0i64;
    for cdigit in str.as_bytes() {
        match state {
            State::LEADING => {
                if *cdigit == ' ' as u8 { continue; }
                if *cdigit == '+' as u8 || *cdigit == '-' as u8 {
                    state = State::DIGITS;
                    sign = Some(if *cdigit == '+' as u8 {true} else {false});
                } else if *cdigit >= '0' as u8 && *cdigit <= '9' as u8 {
                    state = State::DIGITS;
                    res = res * 10;
                    res = res + (*cdigit - '0' as u8) as i64;
                } else { return 0; }
            },
            State::DIGITS => {
                if *cdigit >= '0' as u8 && *cdigit <= '9' as u8 {
                    res = res * 10;
                    res = res + (*cdigit - '0' as u8) as i64;
                    if res > std::i32::MAX as i64 { break; }
                } else { break; }
            },
            _ => { return 0; }
        }
    }
    if sign.is_some() && !sign.unwrap() { res = -res; }
    if res > std::i32::MAX as i64 { return std::i32::MAX; }
    else if res < std::i32::MIN as i64 { return std::i32::MIN; }
    else { return res as i32; }
}