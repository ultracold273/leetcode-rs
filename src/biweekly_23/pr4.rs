
fn max_satisfaction(satisfaction: Vec<i32>) -> i32 {
    let mut satisfaction = satisfaction.clone();
    satisfaction.sort();
    let mut start = -1;
    for (i, s) in satisfaction.iter().enumerate() {
        if *s >= 0 { start = i as i32; break; }
    }

    // quick return
    let mut tsatisfy = 0;
    if start == -1 {
        return 0;
    } else if start > 0 {
        let mut accumulate = (&satisfaction[start as usize..satisfaction.len()]).iter().sum::<i32>();
        start -= 1;
        while start >= 0 {
            if accumulate + satisfaction[start as usize] > 0 {
                accumulate += satisfaction[start as usize];
                start -= 1;
            }
            else { break; }
        }
        start += 1;
    }
    for i in start as usize..satisfaction.len() {
        let time = (i as i32) - start + 1;
        tsatisfy += time * satisfaction[i];
    } 
    tsatisfy
}

#[test]
fn max_satisfaction_test() {
    let satisfaction = vec![-1, -8, 0, 5, -9];
    assert_eq!(max_satisfaction(satisfaction), 14);
}