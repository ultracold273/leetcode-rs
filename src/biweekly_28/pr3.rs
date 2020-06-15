
fn min_sum_of_lengths(arr: Vec<i32>, target: i32) -> i32 {
    let mut res = Vec::new();
    let n = arr.len();
    let mut st = 0;
    let mut ed = 0;
    let mut sum = 0;
    let mut overlap = 0;
    while st < n {
        while sum < target && ed < n {
            sum += arr[ed];
            ed += 1;
        }
        // println!("{} {} {} {} {} {}", st, ed, arr[st], arr[ed], sum, overlap);
        if sum == target {
            if st >= overlap {
                // println!("{} {}", st, ed);
                res.push(ed - st);
                overlap = ed;
            } else if res[res.len() - 1] <= ed - st {
                // overlap = ed;
            } else {
                let last_idx = res.len() - 1;
                res[last_idx] = ed - st;
                overlap = ed;
            }
            sum -= arr[st];
            st += 1;
        // } else if ed == n {
            // break;
        } else {
            sum -= arr[st];
            st += 1;
        }
    }
    res.sort();
    // println!("{:?}", res);
    if res.len() < 2 {
        -1
    } else {
        (res[0] + res[1]) as i32
    }
}

#[test]
fn min_sum_of_lengths_test() {
    // let arr = vec![3,2,2,4,3];
    let arr = vec![
        78, 18, 1, 94, 1, 1, 1, 29, 58, 3, 4, 1, 2, 56, 17, 19, 4, 1, 63, 2, 16, 11, 1, 1, 2, 1,
        25, 62, 10, 69, 12, 7, 1, 6, 2, 92, 4, 1, 61, 7, 26, 1, 1, 1, 67, 26, 2, 2, 70, 25, 2, 68,
        13, 4, 11, 1, 34, 14, 7, 37, 4, 1, 12, 51, 25, 2, 4, 3, 56, 21, 7, 8, 5, 93, 1, 1, 2, 55,
        14, 25, 1, 1, 1, 89, 6, 1, 1, 24, 22, 50, 1, 28, 9, 51, 9, 88, 1, 7, 1, 30, 32, 18, 12, 3,
        2, 18, 10, 4, 11, 43, 6, 5, 93, 2, 2, 68, 18, 11, 47, 33, 17, 27, 56, 13, 1, 2, 29, 1, 17,
        1, 10, 15, 18, 3, 1, 86, 7, 4, 16, 45, 3, 29, 2, 1, 1, 31, 19, 18, 16, 12, 1, 56, 4, 35, 1,
        1, 36, 59, 1, 1, 16, 58, 18, 4, 1, 43, 31, 15, 6, 1, 1, 6, 49, 27, 12, 1, 2, 80, 14, 2, 1,
        21, 32, 18, 15, 11, 59, 10, 1, 14, 3, 3, 7, 15, 4, 55, 4, 1, 12, 4, 1, 1, 53, 37, 2, 5, 72,
        3, 6, 10, 3, 3, 83, 8, 1, 5,
    ];
    let target = 97;
    println!("{}", min_sum_of_lengths(arr, target));
    let arr2 = vec![47,17,4,8,8,2,1,1,8,35,30,1,11,1,1,1,44,1,3,27,2,8];
    let target2 = 88;
    println!("{}", min_sum_of_lengths(arr2, target2));
}
