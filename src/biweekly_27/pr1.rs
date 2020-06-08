
use std::collections::BTreeMap;

fn can_be_equal(target: Vec<i32>, arr: Vec<i32>) -> bool {
    let mut map1 = BTreeMap::new();
    let mut map2 = BTreeMap::new();
    target.iter().for_each(|x| {
        let pcount = map1.entry(*x).or_insert(0);
        *pcount += 1;
    });
    arr.iter().for_each(|x| {
        let pcount = map2.entry(*x).or_insert(0);
        *pcount += 1;
    });
    for (k, v) in map1.iter() {
        if let Some(count) = map2.get(k) {
            if *v != *count { return false; }
        } else {
            return false;
        }
    }
    true
}