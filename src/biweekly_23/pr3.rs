
fn check_inside_circle(r: i32, c_x: i32, c_y: i32, x: i32, y: i32) -> bool {
    (x - c_x) * (x - c_x) + (y - c_y) * (y - c_y) <= r * r
}

fn check_overlap(radius: i32, x_center: i32, y_center: i32, x1: i32, y1: i32, x2: i32, y2: i32) -> bool {
    let inside_circle = |x, y| {check_inside_circle(radius, x_center, y_center, x, y)};
    if inside_circle(x1, y1) || inside_circle(x2, y2) || inside_circle(x1, y2) || inside_circle(x2, y1) {
        true
    } else if x_center >= x1 - radius && x_center <= x2 + radius && y_center >= y1 && y_center <= y2{
        true
    } else if x_center >= x1 && x_center <= x2 && y_center >= y1 - radius && y_center <= y2 + radius {
        true
    } else if x_center >= x1 && x_center <= x2 && y_center >= y1 && y_center <= y2 {
        true
    } else {
        false
    }
}