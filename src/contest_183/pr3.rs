

// An interesting problem
// a = 1, b = 1, c = 7
// First pass: cacbc
fn longest_diverse_string(a: i32, b: i32, c: i32) -> String {
    let charset = vec!['a', 'b', 'c'];
    let mut count = vec![a, b, c];
    let mut index = vec![0, 1, 2];
    let mut sbuilder = Vec::new();

    let mut flag = true;
    while flag {
        index.sort_by(|&a, &b| count[b].cmp(&count[a]));

        flag = false;
        for &i in index.iter() {
            if (sbuilder.len() == 0 || sbuilder[sbuilder.len()-1] != i) &&
                count[i] > 0 {
                    count[i] -= 1;
                    sbuilder.push(i);
                    flag = true;
                    break;
                }
        }
    }

    let mut res = Vec::new();
    // println!("{:?}", count);

    for c in sbuilder {
        res.push(charset[c]);
        if count[c] > 0 {
            res.push(charset[c]);
            count[c] -= 1;
        }
    }
    // if count[0] == 0 && count[1] == 0 && count[2] == 0 {
    res.iter().collect::<String>()
    // } else {
        // "".to_string()
    // }
}

#[test]
fn longest_diverse_string_test() {
    println!("{}", longest_diverse_string(1, 1, 7));
}