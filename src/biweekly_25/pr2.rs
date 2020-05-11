
fn max_diff(num: i32) -> i32 {
    let mut n = num;
    let mut digits = Vec::new();
    while n > 0 { digits.push(n % 10); n /= 10; }
    // change the first non-9 to 9
    let mut max_replace = 9;
    for i in digits.iter().rev() {
        if *i != 9 { max_replace = *i; break; }
    }
    let mut max = 0;
    for i in digits.iter().rev() {
        max *= 10;
        max += if *i == max_replace { 9 } else { *i }
    }
    // println!("{}", max);
    
    let mut min_replace = 0;
    let mut min_replace_to = 0;
    let mut first_digit = 0;
    for (idx, i) in digits.iter().rev().enumerate() {
        if idx == 0 {
            if *i != 1 {
                min_replace = *i;
                min_replace_to = 1;
                break;
            } else { first_digit = *i; }
        } else if idx > 0 && *i != 0 && *i != first_digit {
            min_replace = *i;
            min_replace_to = 0;
            break;
        }
    }
    let mut min = 0;
    for i in digits.iter().rev() {
        min *= 10;
        min += if *i == min_replace { min_replace_to } else { *i }
    }
    // println!("{}", min);
    max - min
}

#[test]
fn max_diff_test() {
    println!("{}", max_diff(1101057));
    // println!("{}", max_diff(555));
    // println!("{}", max_diff(555));
}