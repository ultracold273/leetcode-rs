
fn min_number_of_frogs(croak_of_frogs: String) -> i32 {
    let mut croaks = vec![0u64; 5];
    let mut count = 0;
    for &c in croak_of_frogs.as_bytes() {
        match c as char {
            'c' => {croaks[0] += 1;},
            'r' => {croaks[1] += 1;},
            'o' => {croaks[2] += 1;},
            'a' => {croaks[3] += 1;},
            'k' => {croaks[4] += 1;},
            _ => {},
        }
        if croaks[0] >= croaks[1] && 
           croaks[1] >= croaks[2] &&
           croaks[2] >= croaks[3] &&
           croaks[3] >= croaks[4] {
               let need_frog = croaks[0] - croaks[4];
               count = count.max(need_frog);
        } else { return -1; }
    }
    if croaks[0] == croaks[1] && 
        croaks[1] == croaks[2] &&
        croaks[2] == croaks[3] &&
        croaks[3] == croaks[4] {
        count as i32
    } else {
        -1
    }
}

#[test]
fn frog_test() {
    println!("{}", min_number_of_frogs("ccrrooaakk".to_string()));
}