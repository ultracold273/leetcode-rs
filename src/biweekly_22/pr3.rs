
use std::collections::HashMap;

fn solve(n: i32, m: &mut HashMap<i32, u32>) -> u32 {
    if let Some(&cnt) = m.get(&n) {
        cnt
    } else {
        let cnt;
        if n % 2 == 0 {
            cnt = solve(n / 2, m) + 1;
        } else {
            cnt = solve(3 * n + 1, m) + 1;
        }
        m.insert(n, cnt);
        cnt
    }
}

fn get_kth(lo: i32, hi: i32, k: i32) -> i32 {
    let mut map = HashMap::<i32, u32>::new();
    let mut cnts = Vec::new();
    map.insert(1, 0);
    for i in lo..=hi {
        solve(i, &mut map);
    }

    for i in lo..=hi {
        cnts.push((i, map.get(&i).unwrap().clone()));
    }
    cnts.sort_by(|&a, &b| a.1.cmp(&b.1).then_with(|| a.0.cmp(&b.0)));
    cnts[(k - 1) as usize].0
}