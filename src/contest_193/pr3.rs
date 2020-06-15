fn min_days(bloom_day: Vec<i32>, m: i32, k: i32) -> i32 {
    // binary search
    let mut blooms = bloom_day.clone();
    blooms.sort();
    let mut st = 0;
    let mut ed = blooms.len() - 1;
    let mut ret = -1;
    loop {
        let mid = (st + ed) / 2;
        let day = blooms[mid];
        let mut nbundles = 0;
        let mut nflowers = 0;
        for &d in bloom_day.iter() {
            if d <= day {
                nflowers += 1;
                if nflowers == k as usize {
                    nflowers = 0;
                    nbundles += 1;
                }
            } else {
                nflowers = 0;
            }
        }

        let brk = st == ed;
        if nbundles == m as usize {
            ret = day;
            ed = mid;
        } else if nbundles > m as usize {
            ret = day;
            ed = mid;
        } else {
            if st == mid {
                st += 1;
            } else {
                st = mid;
            }
        }
        if brk {
            break;
        }    }
    ret
}
