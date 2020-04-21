fn min_distance(word1: String, word2: String) -> i32 {
    let m = word1.len();
    let n = word2.len();
    if m == 0 { return n as i32; }
    if n == 0 { return m as i32; }
    let mut dist = vec![vec![0; n+1]; m+1];
    let w1 = word1.chars().collect::<Vec<_>>();
    let w2 = word2.chars().collect::<Vec<_>>();
    for i in 0..=n { dist[0][i] = i; }
    for i in 0..=m { dist[i][0] = i; }
    for i in 1..=m {
        for j in 1..=n {
            if w1[i-1] == w2[j-1] {
                dist[i][j] = dist[i-1][j-1];
            } else {
                let v1 = dist[i-1][j-1] + 1;
                let v2 = dist[i-1][j] + 1;
                let v3 = dist[i][j-1] + 1;
                dist[i][j] = v1.min(v2).min(v3);
            }
        }
    }
    // println!("{:?}", dist);
    dist[m][n] as i32
}

#[test]
fn min_distance_test() {
    println!("{}", min_distance("distance".to_string(), "springbok".to_string()))
}