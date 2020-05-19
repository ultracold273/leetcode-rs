
fn max_power(s: String) -> i32 {
    let mut energy = 0;
    let mut cur_c = 'a';
    let mut cur_energy = 0;
    for (idx, c) in s.chars().enumerate() {
        if idx == 0 { cur_c = c; cur_energy = 1; }
        else if c == cur_c {
            cur_energy += 1;
        } else {
            cur_c = c;
            cur_energy = 1;
        }
        energy = energy.max(cur_energy)
    }
    energy
}