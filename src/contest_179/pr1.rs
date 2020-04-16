fn generate_the_string(n: i32) -> String {
    let mut s = String::new();
    if n % 2 == 1 {
        for _ in 0..n { s.push('a'); }
    } else {
        for _ in 0..n-1 { s.push('a'); }
        s.push('b');
    }
    s
}


#[test]
fn generate_the_string_test() {
    println!("{}", generate_the_string(1));
    println!("{}", generate_the_string(2));
}