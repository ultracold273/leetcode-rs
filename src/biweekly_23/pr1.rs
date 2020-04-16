use std::collections::HashMap;

fn count_largest_group(n: i32) -> i32 {
    let mut digit_sum = 0;
    let mut map = HashMap::<i32, i32>::new();
    for i in 1..=n {
        digit_sum = 0;
        let mut digit = i;
        while digit != 0 { digit_sum += digit % 10; digit /= 10; }
        match map.get_mut(&digit_sum) {
            Some(count) => {*count += 1;},
            None => {map.insert(digit_sum, 1);}
        }
    }

    let mut max_group = 0;
    let mut max_group_count = 0;
    for (_, &v) in map.iter() {
        if v > max_group {
            max_group = v;
            max_group_count = 1;
        } else if v == max_group {
            max_group_count += 1;
        }
    }
    max_group_count
}