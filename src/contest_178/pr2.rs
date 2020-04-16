
use std::collections::HashMap;

fn rank_teams(votes: Vec<String>) -> String {
    let mut team_rankings = HashMap::<char, Vec<usize>>::new();
    if votes.len() == 0 { return "".to_string(); }
    let nteams = votes[0].len();
    for vote in votes {
        for (r, team) in vote.chars().enumerate() {
            let e = team_rankings.entry(team).or_insert(vec![0; nteams]);
            e[r] += 1;
        }
    }

    let mut packing = team_rankings.iter().collect::<Vec<_>>();
    packing.sort_by(|&a, &b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
    packing.iter().map(|&a| a.0).collect::<String>()
}