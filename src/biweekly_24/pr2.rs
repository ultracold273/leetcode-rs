
fn find_number(k: i32, numbers: &Vec<i32>) -> usize {
    let n = numbers.len();
    let mut lo = 0;
    let mut hi = n - 1;
    while lo + 1 < hi {
        let mid = (lo + hi) / 2;
        if numbers[mid] <= k {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    lo
}

fn find_min_fibonacci_numbers(k: i32) -> i32 {
    if k == 1 { return 1; }
    if k == 2 { return 1; }
    let mut fibonacci = Vec::new();
    fibonacci.push(1);
    fibonacci.push(1);
    let mut num = 0;
    while num <= k {
        let n = fibonacci.len();
        num = fibonacci[n-1] + fibonacci[n-2];
        fibonacci.push(num);
    }

    let mut cnt = 0;
    num = k;
    loop {
        let pos = find_number(num, &fibonacci);
        cnt += 1;
        if fibonacci[pos] == num { break; }
        else { num -= fibonacci[pos]; }
    }
    cnt
}

#[test]
fn find_min_fibonacci_numbers_test() {
    println!("{}", find_min_fibonacci_numbers(7));
    println!("{}", find_min_fibonacci_numbers(10));
    println!("{}", find_min_fibonacci_numbers(19));
}