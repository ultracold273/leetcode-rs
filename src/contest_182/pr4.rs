const MOD: u64 = 1000000007;

fn build(cv: &Vec<char>) -> Vec<i32> {
    let n = cv.len();
    let mut next = vec![-1i32; n+1];
    let mut j = next[0];

    for i in 0..n {
        while j >= 0 && cv[i] != cv[j as usize] { j = next[j as usize]; }
        j += 1;
        next[i + 1] = j;
    }
    next
}

fn digit_dfs(n: usize, m: usize, upper: bool, s: &Vec<char>, e: &Vec<char>, next: &Vec<i32>, dp: &mut Vec<Vec<u64>>) -> u64 {
    let mut res = 0u64;
    if m >= e.len() { return 0; }
    if n >= s.len() { return 1; }

    if !upper && dp[n][m] > 0 { return dp[n][m]; }

    let start = 'a' as u8;
    let end = if upper { s[n] as u8 } else { 'z' as u8 };
    for i in start..=end {
        let mut mat = m as i32;
        let c = i as char;
        while mat > 0 && c != e[mat as usize] { mat = next[mat as usize]; }
        if c == e[mat as usize] { mat += 1; }
        let r = digit_dfs(n + 1, mat as usize, upper && c == s[n], s, e, next, dp);
        res = (res + r) % MOD;
    }

    if !upper { dp[n][m] = res; }
    res
}

fn find_good_strings(n: i32, s1: String, s2: String, evil: String) -> i32 {
    let mut dp = vec![vec![0u64; evil.len()]; n as usize];
    let s1v = s1.chars().collect::<Vec<_>>();
    let s2v = s2.chars().collect::<Vec<_>>();
    let evilv = evil.chars().collect::<Vec<_>>();
    let next = build(&evilv);

    let r1 = digit_dfs(0, 0, true, &s1v, &evilv, &next, &mut dp) % MOD;
    let r2 = digit_dfs(0, 0, true, &s2v, &evilv, &next, &mut dp) % MOD;

    let mut res = if r1 > r2 { MOD - r1 + r2 } else {r2 - r1};
    if !s1.contains(&evil) { res = (res + 1) % MOD; }
    res as i32
}