fn all_outside_distance(a: i32, arr: &Vec<i32>, d: i32) -> bool {
    for e in arr.iter() {
        if (a - e).abs() <= d { return false; }
    }
    true
}

fn find_the_distance_value(arr1: Vec<i32>, arr2: Vec<i32>, d: i32) -> i32 {
    let mut cnt = 0;
    for i in arr1 {
        if all_outside_distance(i, &arr2, d) {
            cnt += 1;
        }
    }
    cnt
}