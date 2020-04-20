

fn get_happy_string(n: i32, k: i32) -> String {
    if k > 3 * 2i32.pow(n as u32 - 1) { "".to_string() }
    else {
        let mut chars = Vec::new();
        let mut k = k as u32 - 1;
        let mut n = n as u32;
        match k / 2u32.pow(n - 1) {
            0 => {chars.push('a');},
            1 => {chars.push('b');},
            2 => {chars.push('c');},
            _ => {},
        }
        println!("{:?}", chars);
        k = k % 2u32.pow(n-1);
        n -= 1;
        while n > 0 {
            let prev = chars[chars.len()-1];
            match k / 2u32.pow(n-1) {
                0 => {
                    if prev == 'a' {chars.push('b');}
                    else {chars.push('a');}
                },
                1 => {
                    if prev == 'c' {chars.push('b');}
                    else {chars.push('c');} 
                },
                _ => {},
            }
            k = k % 2u32.pow(n-1);
            n -= 1;
            // println!("{} {}", k, n);
        }
        chars.iter().collect()
    }
}

#[test]
fn get_happy_string_test() {
    println!("{}", get_happy_string(1, 3));
    println!("{}", get_happy_string(1, 4));
    println!("{}", get_happy_string(3, 9));
    // println!("{}", get_happy_string(2, 7));
}