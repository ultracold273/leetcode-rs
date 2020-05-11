
fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
    let mut stack_op = Vec::new();
    let mut tidx = 0;
    for i in 1..=n {
        if tidx >= target.len() { break; }
        stack_op.push("Push".to_string());
        if target[tidx] != i {
            stack_op.push("Pop".to_string());
        } else {
            tidx += 1;
        }
    }
    stack_op
}