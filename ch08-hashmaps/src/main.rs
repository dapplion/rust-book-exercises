use std::collections::HashMap;

fn main() {
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).expect("No value");

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}
