
#[derive(Debug)]
enum Line {
    Normal((f64, f64)),
    Vertical(f64),
}

#[derive(Debug)]
enum LineStatus {
    Overlap,
    Parallel,
    Cross((f64, f64)),
}

fn get_line(p1: &Vec<i32>, p2: &Vec<i32>) -> Line {
    if p1[0] == p2[0] {
        Line::Vertical(p1[0] as f64)
    } else {
        let k = (p1[1] as f64 - p2[1] as f64) / (p1[0] as f64 - p2[0] as f64);
        let b = p1[1] as f64 - k * p1[0] as f64;
        Line::Normal((k, b))
    }
}

fn get_cross(l1: &Line, l2: &Line) -> LineStatus {
    match (l1, l2) {
        (Line::Vertical(x1), Line::Vertical(x2)) => {
            if (x1 - x2).abs() < 10e-6 { LineStatus::Overlap } else { LineStatus::Parallel }
        },
        (Line::Normal((k1, b1)), Line::Vertical(x2)) => {
            LineStatus::Cross((*x2, x2*k1+b1))
        },
        (Line::Vertical(x1), Line::Normal((k2, b2))) => {
            LineStatus::Cross((*x1, x1*k2+b2))
        },
        (Line::Normal((k1, b1)), Line::Normal((k2, b2))) => {
            if (k1 - k2).abs() < 10e-6 {
                if (b1 - b2).abs() < 10e-6 { LineStatus::Overlap } else { LineStatus::Parallel }
            } else {
                let x = (b2 - b1) / (k1 - k2);
                let y = k1 * x + b1;
                LineStatus::Cross((x, y))
            }
        },
    }
}

fn is_on_segment(start: &Vec<i32>, end: &Vec<i32>, pt: &(f64, f64)) -> bool {
    // x direction
    let start_x = start[0].min(end[0]) as f64;
    let end_x = start[0].max(end[0]) as f64;
    pt.0 > start_x - 10e-6 && pt.0 < end_x + 10e-6
}

fn arrange(s1: i32, e1: i32, s2: i32, e2: i32) -> ((i32, i32), (i32, i32)) {
    let s1 = if s1 < e1 { (s1, e1) } else { (e1, s1) };
    let s2 = if s2 < e2 { (s2, e2) } else { (e2, s2) };
    if s1.0 < s2.0 {
        (s1, s2)
    } else {
        (s2, s1)
    }
}

fn intersection(start1: Vec<i32>, end1: Vec<i32>, start2: Vec<i32>, end2: Vec<i32>) -> Vec<f64> {
    let line1 = get_line(&start1, &end1);
    let line2 = get_line(&start2, &end2);
    let lstatus = get_cross(&line1, &line2);
    // println!("line1: {:?} line2: {:?} status: {:?}", line1, line2, lstatus);
    match lstatus {
        LineStatus::Parallel => { Vec::new() },
        LineStatus::Overlap => {
            let mut res = Vec::new();
            let seg_pack;
            match line1 {
                Line::Vertical(_) => {
                    seg_pack = arrange(start1[1], end1[1], start2[1], end2[1]);
                    if (seg_pack.1).0 <= (seg_pack.0).1 {
                        res.push(start1[0] as f64);
                        res.push((seg_pack.1).0 as f64)
                    }
                },
                Line::Normal((k, b)) => {
                    seg_pack = arrange(start1[0], end1[0], start2[0], end2[0]);
                    if (seg_pack.1).0 <= (seg_pack.0).1 {
                        let x = (seg_pack.1).0 as f64;
                        res.push(x);
                        res.push(x*k+b);
                    }
                }
            }
            res
        },
        LineStatus::Cross(pt) => {
            let mut res = Vec::new();
            if is_on_segment(&start1, &end1, &pt) && is_on_segment(&start2, &end2, &pt) {
                res.push(pt.0);
                res.push(pt.1);
            }
            res
        },
    }
}

#[test]
fn intersection_test() {
    println!("{:?}", intersection(vec![1, 1], vec![-1,1], vec![1,0], vec![-3,2]));
}