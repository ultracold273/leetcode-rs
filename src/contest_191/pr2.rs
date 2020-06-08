fn max_dist(length: i32, cuts: &Vec<i32>) -> i32 {
    let mut prev = 0;
    let mut max = 0;
    for cut in cuts.iter() {
        let dist = *cut - prev;
        prev = *cut;
        max = max.max(dist);
    }
    let dist = length - cuts[cuts.len() - 1];
    max.max(dist)
}

fn max_area(h: i32, w: i32, horizontal_cuts: Vec<i32>, vertical_cuts: Vec<i32>) -> i32 {
    let mut horizontal_cuts = horizontal_cuts.clone();
    let mut vertical_cuts = vertical_cuts.clone();
    horizontal_cuts.sort();
    vertical_cuts.sort();

    let max_horizontal = max_dist(h, &horizontal_cuts) as i64;
    let max_vertical = max_dist(w, &vertical_cuts) as i64;
    ((max_horizontal * max_vertical) % 1000000007) as i32
}
