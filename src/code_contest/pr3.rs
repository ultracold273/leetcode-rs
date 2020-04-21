fn get_trigger_time(increase: Vec<Vec<i32>>, requirements: Vec<Vec<i32>>) -> Vec<i32> {
    let n = increase.len();
    let mut accumulate = vec![vec![0, 0, 0]; n + 1];
    for (i, tinc) in increase.iter().enumerate() {
        accumulate[i + 1][0] = accumulate[i][0] + tinc[0];
        accumulate[i + 1][1] = accumulate[i][1] + tinc[1];
        accumulate[i + 1][2] = accumulate[i][2] + tinc[2];
    }
    let binary_find = |val, idx| {
        let mut lo = 0;
        let mut hi = accumulate.len() - 1;
        while lo + 1 < hi {
            let mid = (lo + hi) / 2;
            if accumulate[mid][idx] >= val {
                hi = mid;
            } else {
                lo = mid;
            }
        }
        hi
    };

    let mut time = vec![-1; requirements.len()];
    for (i, r) in requirements.iter().enumerate() {
        let idx = binary_find(r[0], 0).max(binary_find(r[1], 1).max(binary_find(r[2], 2)));
        if r[0] == 0 && r[1] == 0 && r[2] == 0 {
            time[i] = 0;
            continue;
        }
        if accumulate[idx][0] >= r[0] && accumulate[idx][1] >= r[1] && accumulate[idx][2] >= r[2] {
            time[i] = idx as i32;
        }
    }
    time
}
