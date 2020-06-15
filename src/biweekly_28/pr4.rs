
// It seems a bit like clustering algorithm, kmeans

fn update(houses: &Vec<i32>, belonging: &Vec<i32>, k: usize) -> Vec<i32> {
    let mut storage = vec![vec![]; k];
    for (i, kth) in belonging.iter().enumerate() {
        storage[*kth as usize].push(houses[i]);
    }
    let mut res = Vec::new();
    for s in storage {
        let mean = s.iter().sum::<i32>() / s.len() as i32;
        res.push(mean);
    }
    res
}

fn cluster(houses: &Vec<i32>, kpos: &Vec<i32>) -> (i32, Vec<i32>) {
    let mut res = Vec::new();
    let mut dd = 0;
    for h in houses.iter() {
        let mut min = std::i32::MAX;
        let mut belong = 0;
        for (kidx, k) in kpos.iter().enumerate() {
            let dist = (k-h).abs();
            if dist < min {
                min = dist;
                belong = kidx;
            }
        }
        dd += min;
        res.push(belong as i32);
    }
    (dd, res)
}

fn calculate_init_pos(houses: &Vec<i32>, k: i32) -> Vec<i32> {
    let maximum = *houses.iter().max().unwrap();
    let minimum = *houses.iter().min().unwrap();
    let sep = (maximum - minimum) / (k + 2);
    let mut res = Vec::new();
    for i in 0..k {
        res.push(minimum + i*sep);
    }
    res
}

fn min_distance(houses: Vec<i32>, k: i32) -> i32 {
    let mut kpos = calculate_init_pos(&houses, k);
    let mut prev_dist = -1;
    loop {
        let (dist, belongs) = cluster(&houses, &kpos);
        if prev_dist == dist {
            break;
        }
        prev_dist = dist;
        kpos = update(&houses, &belongs, k as usize);
    }
    prev_dist
}


// rec[i][j] stores the distance sum when a postbox is serviced
// i-th houses to j-th house
// dp[i][j] is the min distance with first i-th houses and j postboxes
// dp[i][j] = min(dp[k-1][j-1] + rec[k][i], dp[i][j]) for k in j-1 to i
fn min_distance_dp(houses: Vec<i32>, k: i32) -> i32 {
    let mut houses = houses;
    let k = k as usize;
    let mut n = houses.len();
    houses.sort();

    let mut range = vec![vec![0; n]; n];
    for i in 0..n {
        for j in i..n {
            let mid = (i + j) / 2;
            for kk in i..=j {
                range[i][j] += (houses[kk] - houses[mid]).abs();
            }
        }
    }
    // println!("{:?}", range);

    let mut dp = vec![vec![std::i32::MAX; k + 1]; n];
    for i in 0..n { dp[i][1] = range[0][i]; }
    for i in 0..n-1 {
        for j in 1..k {
            if j > i + 1 { continue; }
            for p in i+1..n {
                dp[p][j+1] = dp[p][j+1].min(dp[i][j] + range[i+1][p]);
            }
        }
    }
    // println!("{:?}", dp);
    
    dp[n-1][k]
}