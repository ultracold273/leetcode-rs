
const MOD: i64 = 1000000000 + 7;

fn calulate(src: &Vec<i64>, dst: &mut Vec<i64>) {
    let transfer = vec![
        vec![4,5,7,8,9],
        vec![4,6,7,8],
        vec![4,5,8,9,11],
        vec![5,9,10,11],
        vec![0,1,2,10,11],
        vec![0,2,3,10],
        vec![1,8,9,11],
        vec![1,2,9,10,11],
        vec![0,1,2,6],
        vec![0,2,3,6,7],
        vec![3,4,5,7],
        vec![2,3,4,6,7],
    ];
    for i in 0..12 {
        dst[i] = 0;
        for index in transfer[i].iter() {
            dst[i] = (dst[i] + src[*index]) % MOD;
        }
    }
}

fn num_of_ways(n: i32) -> i32 {
    let mut dp1 = vec![1; 12];
    let mut dp2 = vec![0; 12];
    if n == 1 {
        (dp1.iter().sum::<i64>() % MOD) as i32
    } else {
        for i in 2..=n {
            if i % 2 == 0 {
                calulate(&dp1, &mut dp2);
            } else {
                calulate(&dp2, &mut dp1);
            }
        }
        if n % 2 == 0 {
            (dp2.iter().sum::<i64>() % MOD) as i32
        } else {
            (dp1.iter().sum::<i64>() % MOD) as i32
        }
    }
}