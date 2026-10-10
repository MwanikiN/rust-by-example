use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the number of player names
    let n: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Create a mutable HashMap to store player scores
    let mut player_scores: HashMap<String, i32> = HashMap::new();
    
    // TODO: Write your code below
    // Read n player names and use .entry().or_insert(100) to add them to the map
    for _ in 0..n {
        let name: String = lines.next().unwrap().unwrap().trim().to_string();
        player_scores.entry(name).or_insert(100);
    }
    
    // Print each player and their score in the format: [name]: [score]
    for (player, score) in &player_scores {
        println!("{}: {}", player, score);
    }
}