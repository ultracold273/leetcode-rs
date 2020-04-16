
fn sort_by_bits(arr: Vec<i32>) -> Vec<i32> {
    let mut ones = Vec::new();
    for n in arr {
        let mut count = 0;
        let mut number = n;
        while number > 0 { count += number & 1; number = number >> 1; }
        ones.push((count, n));
    }
    
    ones.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
    ones.iter().map(|a| a.1).collect()
}