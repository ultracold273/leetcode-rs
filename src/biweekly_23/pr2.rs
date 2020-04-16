
use std::collections::HashMap;

fn can_construct(s: String, k: i32) -> bool {
    let cv = s.chars().collect::<Vec<_>>();
    let mut map = HashMap::<char, i32>::new();
    
    for c in cv {
        match map.get_mut(&c) {
            Some(count) => {*count += 1;},
            None => {map.insert(c, 1);}
        }
    }
    
    let mut odd_char_count = 0;
    for (_, &count) in map.iter() {
        if count % 2 == 1 { odd_char_count += 1;}
    }
    
    return s.len() >= k as usize && odd_char_count <= k
}