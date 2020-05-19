use std::collections::{HashSet, HashMap};

fn people_indexes(favorite_companies: Vec<Vec<String>>) -> Vec<i32> {
    let mut company_map = HashMap::new();
    for (idx, companies) in favorite_companies.iter().enumerate() {
        for company in companies.iter() {
            let users = company_map.entry(company).or_insert(HashSet::new());
            users.insert(idx);
        }
    }

    let mut res = Vec::new();
    for (idx, companies) in favorite_companies.iter().enumerate() {
        let mut set = company_map[&companies[0]].clone();
        for i in 1..companies.len() {
            let users = &company_map[&companies[i]];
            set = set.intersection(users).map(|n| *n).collect();
        }
        if set.len() == 1 {
            res.push(idx as i32);
        }
    }
    res
}