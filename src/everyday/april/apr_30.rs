use std::collections::HashSet;

fn digit_sum(mut n: i32) -> i32 {
    let mut res = 0;
    while n != 0 { res += (n % 10) * (n % 10); n = n / 10; }
    res
}

fn is_happy(n: i32) -> bool {
    let mut n = n;
    let mut set = HashSet::new();
    while n != 1 && !set.contains(&n) {
        set.insert(n);
        n = digit_sum(n);
    }
    if n == 1 { true } else { false }
}