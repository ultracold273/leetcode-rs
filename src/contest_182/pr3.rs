
use std::collections::HashMap;

struct UndergroundSystem {
    db_i: HashMap<String, Vec<(i32, i32)>>,
    db_o: HashMap<String, Vec<(i32, i32)>>,
}

impl UndergroundSystem {

    fn new() -> Self {
        UndergroundSystem { db_i: HashMap::new(), db_o: HashMap::new() }
    }
    
    fn check_in(&mut self, id: i32, station_name: String, t: i32) {
        if let Some(v) = self.db_i.get_mut(&station_name) {
            v.push((id, t));
        } else {
            self.db_i.insert(station_name, vec![(id, t)]);
        }
    }
    
    fn check_out(&mut self, id: i32, station_name: String, t: i32) {
        if let Some(v) = self.db_o.get_mut(&station_name) {
            v.push((id, t));
        } else {
            self.db_o.insert(station_name, vec![(id, t)]);
        }
    }
    
    fn get_average_time(&mut self, start_station: String, end_station: String) -> f64 {
        let v_i = self.db_i.get_mut(&start_station).unwrap();
        let v_o = self.db_o.get_mut(&end_station).unwrap();
        let mut npassenger = 0f64;
        let mut time = 0f64;
        v_i.sort_by(|&a, &b| a.0.cmp(&b.0));
        v_o.sort_by(|&a, &b| a.0.cmp(&b.0));

        let m = v_i.len();
        let n = v_o.len();
        let mut j = 0;
        for i in 0..m {
            while j < n && v_o[j].0 < v_i[i].0 { j += 1; }
            if j == n { break; }
            else if v_o[j].0 == v_i[i].0 { 
                npassenger += 1f64; 
                time += (v_o[j].1 - v_i[i].1) as f64;
            } else { continue; }
        }
        time / npassenger
    }
}
