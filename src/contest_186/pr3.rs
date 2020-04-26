
use std::collections::BTreeMap;

fn find_diagonal_order(nums: Vec<Vec<i32>>) -> Vec<i32> {
    let mut map = BTreeMap::new();
    for (i, num) in nums.iter().enumerate() {
        for (j, &val) in num.iter().enumerate() {
            let diag_line = map.entry(i + j).or_insert(Vec::new());
            diag_line.insert(0, val);
        }
    }
    let mut res = Vec::new();
    for (_, v) in map.iter() {
        res.extend(v);
    }
    res
}

#[test]
fn find_diagonal_order_test() {
    // let nums = vec![
    //     vec![1,2,3,4,5],
    //     vec![6,7],
    //     vec![8],
    //     vec![9,10,11],
    //     vec![12,13,14,15,16]
    // ];
    let nums = vec![
        vec![1,2,3,4,5],
        vec![6,7],
        vec![8],
        vec![9,10,11],
        vec![12,13,14,15,16]
    ];
    println!("{:?}", find_diagonal_order(nums));
}