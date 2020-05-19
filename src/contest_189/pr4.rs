use std::f64::consts::PI;

fn num_points(points: Vec<Vec<i32>>, r: i32) -> i32 {
    // A naive algorithm gives O(n^3), every two points determines at most
    // two circles, we need to check the left n - 2 points inside the circle.
    // Angular sweep algorithm, sweep the angles
    let n = points.len();
    let mut max_count = 0;
    for i in 0..n {
        let p = (points[i][0] as f64, points[i][1] as f64);
        let mut angles = Vec::new();
        for j in 0..n {
            if i == j {
                continue;
            }
            let q = (points[j][0] as f64, points[j][1] as f64);
            // Calculate the angle between PQ and PC when Q is on circumference
            let dist = ((q.1 - p.1).powi(2) + (q.0 - p.0).powi(2)).sqrt();
            if dist > (2.0 * r as f64) {
                continue;
            }
            let b = (dist / (2.0 * r as f64)).acos();

            // Calculate the angle between PQ and x-axis
            let mut a = ((q.1 - p.1) / (q.0 - p.0)).atan();
            if q.1 <= p.1 && q.0 < p.0 { a -= PI; } // devil's details
            if q.1 > p.1 && q.0 < p.0 { a += PI; }
            angles.push((a - b, a + b))
        }
        // f64 cannot be directly sort as they are not equal
        angles.sort_by(|a, b| a.partial_cmp(b).unwrap());

        // rotation, double it
        let mut db_angle = Vec::new();
        angles
            .iter()
            .for_each(|(a, b)| db_angle.push((*a + 2.0 * PI, *b + 2.0 * PI)));
        angles.extend(db_angle.iter());
        let mut mcount = 1;
        let mut oqueue = Vec::new();
        for angle in angles.iter() {
            while oqueue.len() > 0 && oqueue[0] < angle.0 {
                oqueue.remove(0);
            }
            let mut nn = oqueue.len();
            oqueue.push(0f64);
            while nn > 0 && oqueue[nn - 1] > angle.1 {
                oqueue[nn] = oqueue[nn - 1];
                nn -= 1;
            }
            oqueue[nn] = angle.1;
            mcount = mcount.max(oqueue.len() + 1);
        }
        max_count = max_count.max(mcount)
    }
    max_count as i32
}
