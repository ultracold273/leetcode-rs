
use std::collections::{BTreeMap, BTreeSet};

fn display_table(orders: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut table_meals = BTreeMap::<i32, BTreeMap<String, i32>>::new();
    let mut meals = BTreeSet::new();
    for order in orders.iter() {
        let table = order[1].parse::<i32>().unwrap();
        let meal_map = table_meals.entry(table).or_insert(BTreeMap::new());
        let meal_count = meal_map.entry(order[2].clone()).or_insert(0);
        *meal_count += 1;
        meals.insert(order[2].clone());
    }

    let mut res = Vec::new();
    let mut title = Vec::new();
    title.push(String::from("Table"));
    for m in meals.iter() {
        title.push(m.to_string());
    }
    res.push(title);
    for (&table, meal_map) in table_meals.iter() {
        let mut r = Vec::new();
        r.push(table.to_string());
        for m in meals.iter() {
            r.push(meal_map.get(m).unwrap_or(&0).to_string());
        }
        res.push(r);
    }
    res
}