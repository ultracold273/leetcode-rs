
use std::collections::HashMap;

fn dest_city(paths: Vec<Vec<String>>) -> String {
    let mut map = HashMap::<String, String>::new();
    for path in paths.iter() {
        let start = path[0].clone();
        let end = path[1].clone();
        map.insert(start, end);
    }
    let mut s = &paths[0][0];
    loop {
        if let Some(a) = map.get(s) {
            s = a;
        } else {
            return s.to_string();
        }
    }
    // "0".to_string()
}