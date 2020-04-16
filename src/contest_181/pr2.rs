fn divisors_and_sums(n: i32) -> (i32, i32) {
    let mut count = 0;
    let mut sum = 0;
    let mut i = 1;
    while i * i <= n {
        if n % i == 0 {
            if n / i != i {
                count += 2;
                sum += i;
                sum += n / i;
            } else {
                count += 1;
                sum += i;
            }
        }
        i += 1;
    }
    (count, sum)
}

pub fn sum_four_divisors(nums: Vec<i32>) -> i32 {
    let mut sum = 0;
    for num in nums {
        let (cnt, acc) = divisors_and_sums(num);
        if cnt == 4 {
            sum += acc;
        }
    }
    sum
}