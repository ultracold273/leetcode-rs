
use std::collections::BTreeMap;

fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
    let mut longest = 0;
    let mut map = BTreeMap::new();
    let mut st = 0;
    for ed in 0..nums.len() {
        let pcnt = map.entry(nums[ed]).or_insert(0);
        *pcnt += 1;
        // map.insert(nums[ed]);
        loop {
            let pmin = map.iter().next().unwrap().0;
            let pmax = map.iter().rev().next().unwrap().0;
            if *pmax - *pmin <= limit { break; }
            let pstcnt = map.get_mut(&nums[st]).unwrap();
            *pstcnt -= 1;
            if *pstcnt == 0 {
                map.remove(&nums[st]);
            }
            // map.remove(&nums[st]);
            st += 1;
        }
        longest = longest.max(ed - st + 1);
    }
    longest as i32
}