
// Leading zeros are messy

fn find_one(s: &Vec<i32>, modulus: i32) -> Option<usize> {
    for (idx, m) in s.iter().enumerate().rev() {
        if *m == modulus { return Some(idx) }
    }
    None
}

fn find_two_nonzero(s: &Vec<i32>) -> Option<(usize, usize)> {
    let mut nz1 = None;
    let mut nz2 = None;
    for (idx, m) in s.iter().enumerate().rev() {
        if *m != 0 {
            if nz1.is_none() { nz1 = Some(idx);}
            else if nz2.is_none() { nz2 = Some(idx); break; }
        }
    }
    nz1.and_then(|x| nz2.and_then(|y| Some((x, y))))
}

fn largest_multiple_of_three(digits: Vec<i32>) -> String {
    let mut digits = digits.clone();
    if digits.iter().fold(true, |a, &b| a && b == 0) { return "0".to_string(); }
    let sum = digits.iter().sum::<i32>();
    digits.sort_by(|a, b| b.cmp(&a));
    if sum % 3 != 0 {
        let modulus = (0..digits.len()).map(|i| digits[i] % 3).collect::<Vec<_>>();
        if let Some(pos) = find_one(&modulus, sum % 3) {
            digits.remove(pos);
        } else if let Some((p1, p2)) = find_two_nonzero(&modulus){
            digits.remove(p1);
            digits.remove(p2);
        } else {
            return "".to_string()
        }
    }
    if digits.iter().fold(true, |a, &b| a && b == 0) { return "".to_string() }
    digits.iter().map(|a| a.to_string()).collect::<String>()
}

#[test]
fn largest_multiple_of_three_test() {
    println!("{}", largest_multiple_of_three(vec![8,1,9]));
    println!("{}", largest_multiple_of_three(vec![8,6,7,1,0]));
    println!("{}", largest_multiple_of_three(vec![1]));
    println!("{}", largest_multiple_of_three(vec![1,0,0]));
    println!("{}", largest_multiple_of_three(vec![0,0,0,0]));
}