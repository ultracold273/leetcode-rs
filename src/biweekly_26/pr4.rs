use std::collections::BTreeMap;
fn cmp(a: &String, b: &String) -> bool {
    if a.len() > b.len() {
        true
    } else if a.len() < b.len() {
        false
    } else {
        let mut res = true;
        for (c1, c2) in a.chars().zip(b.chars()) {
            if c1 > c2 {
                res = true;
                break;
            } else if c1 < c2 {
                res = false;
                break;
            }
        }
        res
    }
}

fn make(orig: &String, number: usize) -> String {
    let mut res = Vec::new();
    let mut inserted = false;
    let cnumber = (48 + number as u8) as char;
    for c in orig.chars() {
        if !inserted && c <= cnumber {
            inserted = true;
            res.push(cnumber);
        }
        res.push(c);
    }
    if !inserted {
        res.push(cnumber);
    }
    res.iter().collect()
}

fn largest_number(cost: Vec<i32>, target: i32) -> String {
    let mut cost_map = BTreeMap::new();
    for (i, c) in cost.iter().enumerate() {
        cost_map.insert(*c, i + 1);
    }
    let mut costs = cost_map.iter().map(|(c, n)| (*c, *n)).collect::<Vec<_>>();
    costs.sort();

    let mut dp: Vec<String> = Vec::new();
    dp.push("0".to_string());
    for i in 1..=target {
        // dp[i as usize] = "0".to_string();
        dp.push("0".to_string());
        for &(cost, number) in costs.iter() {
            if cost > i {
                break;
            } else if cost == i {
                let nstr = format!("{}", number);
                if !cmp(&dp[i as usize], &nstr) {
                    dp[i as usize] = nstr;
                }
            } else {
                let pos = (i - cost) as usize;
                if dp[pos] == "0".to_string() {
                    continue;
                }
                let new_digit = make(&dp[pos], number);
                if !cmp(&dp[i as usize], &new_digit) {
                    dp[i as usize] = new_digit;
                }
            }
        }
    }
    // "0".to_string()
    dp[target as usize].clone()
}

#[test]
fn largest_number_test() {
    let cost = vec![4, 3, 2, 5, 6, 7, 2, 5, 5];
    let target = 9;
    println!("{}", largest_number(cost, target));

    let cost2 = vec![7, 6, 5, 5, 5, 6, 8, 7, 8];
    let target2 = 12;
    println!("{}", largest_number(cost2, target2));

    let cost3 = vec![2, 4, 6, 2, 4, 6, 4, 4, 4];
    let target3 = 5;
    println!("{}", largest_number(cost3, target3));

    let cost4 = vec![6, 10, 15, 40, 40, 40, 40, 40, 40];
    let target4 = 47;
    println!("{}", largest_number(cost4, target4));

    let cost5 = vec![70, 84, 55, 63, 74, 44, 27, 76, 34];
    let target5 = 659;
    println!("{}", largest_number(cost5, target5));
}
