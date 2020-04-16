
fn stone_game_dfs(index: usize, sv: &Vec<i32>, best_strategy: &mut Vec<Option<(i32, i32)>>) -> (i32, i32) {
    if index >= sv.len() {
        (0, 0)
    } else if let Some(pick) = best_strategy[index] {
        pick
    } else if index == sv.len() - 1 {
        let pick = (sv[index], 0);
        best_strategy[index] = Some(pick);
        pick
    } else if index == sv.len() - 2 {
        let (adv, my) = stone_game_dfs(index+1, sv, best_strategy);
        let pick1 = (sv[index] + my, adv);
        let pick2 = (sv[index] + sv[index+1], 0);
        let max_pick = if pick1.0 > pick2.0 { pick1 } else { pick2 };
        best_strategy[index] = Some(max_pick);
        max_pick
    } else {
        let other_pick1 = stone_game_dfs(index+1, sv, best_strategy);
        let other_pick2 = stone_game_dfs(index+2, sv, best_strategy);
        let other_pick3 = stone_game_dfs(index+3, sv, best_strategy);
        let pick1 = (sv[index] + other_pick1.1, other_pick1.0);
        let pick2 = (sv[index] + sv[index+1] + other_pick2.1, other_pick2.0);
        let pick3 = (sv[index] + sv[index+1] + sv[index+2] + other_pick3.1, other_pick3.0);
        let max_pick = if pick1.0 > pick2.0 && pick1.0 > pick3.0 { pick1 } else if pick2.0 > pick3.0 { pick2 } else { pick3 };
        best_strategy[index] = Some(max_pick);
        max_pick
    }
}

fn stone_game_iii(stone_value: Vec<i32>) -> String {
    let mut dp :Vec<Option<(i32, i32)>> = vec![None; stone_value.len()];
    let (alice, bob) = stone_game_dfs(0, &stone_value, &mut dp);
    if alice > bob {
        "Alice".to_string()
    } else if alice == bob {
        "Tie".to_string()
    } else {
        "Bob".to_string()
    }
}