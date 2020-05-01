
fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut intervals = intervals.clone();
    intervals.sort_by(|a, b| a[0].cmp(&b[0]));
    let mut res = Vec::new();
    let n = intervals.len();
    res.push(intervals[0].clone());
    for i in 1..n {
        let nres = res.len();
        if intervals[i][0] <= res[nres-1][1] {
            res[nres-1][1] = res[nres-1][1].max(intervals[i][1]);
        } else {
            res.push(intervals[i].clone())
        }
    }
    res
}