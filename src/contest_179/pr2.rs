
fn num_times_all_blue(light: Vec<i32>) -> i32 {
    let mut ranges = Vec::<(i32, i32)>::new();
    let mut cnt = 0;
    for bulb in light {
        let mut pos = ranges.len();
        // If we use binary search, we would get faster
        for (i, (rl, _)) in ranges.iter().enumerate() {
            if *rl > bulb { pos = i; break; }
        }
        // Merge ranges
        if pos > 0 && ranges[pos-1].1 + 1 == bulb { ranges[pos-1].1 = bulb; }
        else if pos < ranges.len() && ranges[pos].0 == bulb + 1 { ranges[pos].0 = bulb; }
        else { ranges.insert(pos, (bulb, bulb)); }

        if pos > 0 && pos < ranges.len() && ranges[pos-1].1 == ranges[pos].0 - 1 { 
            let right = ranges[pos].1;
            ranges.remove(pos); 
            ranges[pos-1].1 = right;
        }
        if ranges.len() == 1 && ranges[0].0 == 1 { cnt += 1; }
    }
    cnt
}

#[test]
fn num_times_all_blue_test() {
    let tcases = vec![
        (vec![1,8,3,4,9,6,7,2,5,10], 3),
        // (vec![2,1,3,5,4], 3), 
        // (vec![3,2,4,1,5], 2), 
        // (vec![4,1,2,3], 1), 
        // (vec![2,1,4,3,6,5], 3),
        // (vec![1,2,3,4,5,6], 6),
    ];
    for (case, res) in tcases {
        assert_eq!(num_times_all_blue(case), res);
    }
}