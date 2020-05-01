
fn trap(height: Vec<i32>) -> i32 {
    let mut water = 0;
    let mut stack = (0, 0);
    
    let mut pool = 0;
    for (i, &h) in height.iter().enumerate() {
        if i == 0 { stack = (i, h); }
        else if h < stack.1 {
            pool += stack.1 - h;
        } else {
            stack = (i, h);
            water += pool;
            pool = 0;
        }
    }
    
    pool = 0;
    for (i, &h) in height.iter().rev().enumerate() {
        // println!("{} {}", i, h);
        if i == 0 { stack = (i, h); }
        else if h <= stack.1 {
            pool += stack.1 - h;
        } else {
            stack = (i, h);
            water += pool;
            pool = 0;
        }
    }
    
    water
}