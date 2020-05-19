fn relative_prime(nomin: i32, denomin: i32) -> bool {
    let mut n = nomin;
    let mut d = denomin;
    while d % n != 0 {
        let temp = d % n;
        d = n;
        n = temp;
    }
    n == 1
}

fn fractions(denominator: i32, res: &mut Vec<String>) {
    for nominator in 1..denominator {
        if relative_prime(nominator, denominator) {
            res.push(format!("{}/{}", nominator, denominator));
        }
    }
}

fn simplified_fractions(n: i32) -> Vec<String> {
    let mut res = Vec::new();
    for i in 2..=n {
       fractions(i, &mut res); 
    }
    res
}