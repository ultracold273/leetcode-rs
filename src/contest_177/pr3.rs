fn divisor(num: i32) -> Vec<i32> {
    let mut d = (num as f64).sqrt() as i32;
    while num % d != 0 { d -= 1; }
    vec![d, num / d]
}

fn closest_divisors(num: i32) -> Vec<i32> {
    let v1 = divisor(num + 1);
    let v2 = divisor(num + 2);
    let absv1 = v1[1] - v1[0];
    let absv2 = v2[1] - v2[0];
    if absv1 < absv2 { v1 } else if absv1 == absv2 { [v1, v2].concat() } else { v2 }
}