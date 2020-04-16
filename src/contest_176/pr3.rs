
use std::collections::{HashSet, HashMap};

fn max_events(events: Vec<Vec<i32>>) -> i32 {
    let mut set = HashSet::<usize>::new();
    let mut start = HashMap::<i32, Vec<usize>>::new();
    let mut end = HashMap::<i32, Vec<usize>>::new();
    let mut eday = 0;
    for (id, event) in events.iter().enumerate() {
        let vs = start.entry(event[0]).or_insert(Vec::new());
        vs.push(id);
        let vd = end.entry(event[1]+1).or_insert(Vec::new());
        vd.push(id);
        eday = eday.max(event[1]+1);
    }

    let mut count = 0;
    for day in 1..eday {
        if let Some(vs) = start.get(&day) {
            vs.iter().for_each(|&a| {set.insert(a);});
        }
        if let Some(ve) = end.get(&day) {
            ve.iter().for_each(|&a| if set.contains(&a) {set.remove(&a);} )
        }
        println!("{}: {:?}", day, set);
        let mut end_day = std::i32::MAX;
        let mut fid = None;
        for id in &set {
            if events[*id][1] < end_day {
                end_day = events[*id][1];
                fid = Some(*id);
            }
        }
        if let Some(id) = fid {
            count += 1;
            set.remove(&id);
        }
    }
    count
}

#[test]
fn max_events_test() {
    println!("{}", max_events(vec![vec![1,2],vec![2,3],vec![3,4],vec![1,2]]));
}

// Oops
// fn max_events(events: Vec<Vec<i32>>) -> i32 {
//     let mut events = events.clone();
//     let mut dp = vec![0i32; events.len()];

//     events.sort_by(|a, b| a[1].cmp(&b[1]));
//     dp[0] = 1;
//     for i in 1..events.len() {
//         // events[i][0]
//         let mut lo = 0;
//         let mut hi = i - 1;
//         while lo + 1 < hi {
//             let u = (lo + hi) / 2;
//             if events[u][1] <= events[i][0] {
//                 lo = u;
//             } else {
//                 hi = u;
//             }
//         }
//         if events[lo][1] <= events[i][0] {
//             dp[i] = dp[lo] + 1;
//         } else {
//             dp[i] = dp[lo-1] + 1;
//         }
//     }
//     *(dp.iter().max().unwrap())
// }