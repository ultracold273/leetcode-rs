
// pathsum problem

use std::collections::HashMap;

fn max_pathsum(hid: i32, map: &HashMap<i32, Vec<i32>>, time: &Vec<i32>) -> i32 {
    match map.get(&hid) {
        Some(children) => {
            children.iter()
                    .map(|&child| max_pathsum(child, map, time))
                    .max().unwrap() + time[hid as usize]
        },
        None => { time[hid as usize] },
    }
}

fn num_of_minutes(n: i32, head_id: i32, manager: Vec<i32>, inform_time: Vec<i32>) -> i32 {
    let mut map = HashMap::<i32, Vec<i32>>::new();
    for (child, parent) in manager.iter().enumerate() {
        match map.get_mut(parent) {
            Some(v) => {v.push(child as i32);},
            None => {map.insert(*parent, vec![child as i32]);}
        }
    }
    max_pathsum(head_id, &map, &inform_time)
}