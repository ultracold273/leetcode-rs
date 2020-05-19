fn busy_student(start_time: Vec<i32>, end_time: Vec<i32>, query_time: i32) -> i32 {
    let n = start_time.len();
    let mut count = 0;
    for i in 0..n {
        if query_time >= start_time[i] && query_time <= end_time[i] {
            count += 1;
        }
    }
    count
}
