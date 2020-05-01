fn max_area(height: Vec<i32>) -> i32 {
    if height.len() == 0 { return 0; }
    let mut st = 0;
    let mut ed = height.len() - 1;
    let mut area = (height.len() as i32 - 1) * height[st].min(height[ed]);
    while st < ed {
        if height[st] < height[ed] {
            st += 1;
        } else {
            ed -= 1;
        }
        let cur_area = (ed - st) as i32 * height[st].min(height[ed]);
        area = area.max(cur_area);
    }
    area
}