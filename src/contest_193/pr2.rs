use std::collections::HashMap;

fn find_least_num_of_unique_ints(arr: Vec<i32>, k: i32) -> i32 {
    let mut hmap = HashMap::new();
    for item in arr.iter() {
        let e = hmap.entry(item).or_insert(0);
        *e += 1;
    }
    let mut counts = vec![0; arr.len()];
    let mut sum = 0;
    for (_, &count) in hmap.iter() {
        counts[count as usize - 1] += 1;
        sum += 1;
    }
    // println!("{:?} {}", counts, sum);

    let mut k = k;
    let mut st = 0i32;
    loop {
        k -= (st + 1) * counts[st as usize];
        if k == 0 {
            sum -= counts[st as usize];
            break;
        } else if k < 0 {
            // println!("{}", sum);
            k += (st + 1) * counts[st as usize];
            sum -= k / (st + 1);
            // println!("{}", sum);
            break;
        } else {
            sum -= counts[st as usize]
        }
        st += 1;
    }

    sum
}
