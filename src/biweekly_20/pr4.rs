
const MOD: u64 = 1000000000 + 7;

fn count_orders(n: i32) -> i32 {
    let mut res = 1u64;
    for i in 1..=n {
        let i = i as u64;
        res = (res * i * (2 * i - 1)) % MOD;
    }
    res as i32
}