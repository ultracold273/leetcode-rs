
const MOD: u64 = 1000000000 + 7;

fn number_ways(hats: Vec<Vec<i32>>) -> i32 {
    let n = hats.len();
    let mut hatmap = vec![Vec::new(); 40];
    for (idx, hs) in hats.iter().enumerate() {
        hs.iter().for_each(|&h| hatmap[h as usize - 1].push(idx));
    }
    let mut dp = vec![0u64; 1<<n];
    dp[0] = 1;

    for h in 0..40 {
        for s in (0..(1<<n)).rev() {
            for &k in hatmap[h].iter() {
                if s & (1 << k) != 0 { continue }
                dp[s + (1 << k)] = (dp[s + (1<<k)] + dp[s]) % MOD;
            }
        }
    }
    // println!("{:?}", dp);
    dp[(1<<n)-1] as i32
}

#[test]
fn number_ways_test() {
    let hats = vec![vec![3,4],vec![4,5],vec![5]];
    println!("{}", number_ways(hats));
}