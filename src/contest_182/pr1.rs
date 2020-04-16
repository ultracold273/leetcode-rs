
use std::collections::HashMap;

fn find_lucky(arr: Vec<i32>) -> i32 {
    let mut map = HashMap::<i32, i32>::new();
    for n in arr {
        if let Some(c) = map.get_mut(&n) {
            *c += 1;
        } else {
            map.insert(n, 1);
        }
    }

    let mut lucky = -1;
    for (&i, &c) in map.iter() {
        if i == c && i > lucky{
            lucky = i;
        }
    }
    lucky
}