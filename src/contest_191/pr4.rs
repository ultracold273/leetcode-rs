
// Calculate combination
fn C(a: usize, b: usize) -> f64 {
    let denominator = (1..=b).fold(1f64, |prod, i| prod * i as f64);
    let nominator = ((a-b+1)..=a).fold(1f64, |prod, i| prod * i as f64);
    nominator / denominator
}

// dp[c][i][p][q]
//  * c - color assigned to the first box
//  * i - number of balls in the first box
//  * p - number of colors of the balls in the first box
//  * q - number of colors of the balls in the second box

fn get_probability(balls: Vec<i32>) -> f64 {
    let ncolor = balls.len();
    let nballs = (balls.iter().sum::<i32>() / 2) as usize;

    let mut dp = vec![vec![vec![vec![0f64; ncolor + 1]; ncolor + 1]; nballs + 1]; ncolor+1];
    dp[0][0][0][0] = 1.0;

    let mut sum = 0;
    for c in 0..ncolor {
        let nb = balls[c] as usize;
        for i in 0..=nballs {
            for p in 0..=ncolor {
                for q in 0..=ncolor {
                    if !(dp[c][i][p][q] > 0.0) { continue; }

                    let mut total = 0f64;
                    for l in 0..=nb {
                        if i + l <= nballs && sum - i + nb - l <= nballs {
                            total += C(nballs - i, l) * C(nballs - (sum - i), nb - l);
                        }
                    }

                    for l in 0..=nb {
                        let pp = if l == 0 { p } else { p + 1 };
                        let qq = if l == nb { q } else { q + 1 };
                        if i + l <= nballs && sum - i + nb - l <= nballs {
                            let v = C(nballs - i, l) * C(nballs - (sum - i), nb - l) / total;
                            dp[c + 1][i + l][pp][qq] += dp[c][i][p][q] * v;
                        }
                    }
                }
            }
        }
        sum += nb;
    }

    // println!("{:?}", dp);

    let mut res = 0f64;
    for p in 0..=ncolor {
        res += dp[ncolor][nballs][p][p];
    }
    res
}

#[test]
fn get_probability_test() {
    println!("{}", get_probability(vec![2,1,1]));
}
