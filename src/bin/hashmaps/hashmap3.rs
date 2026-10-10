use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the number of student-score pairs
    let n: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Create a mutable hash map to store student names and scores
    let mut students: HashMap<String, i32> = HashMap::new();
    
    // TODO: Write your code below
    // Read n pairs of inputs (student name and score) and insert them into the hash map
    
    for _ in 0..n {
        let name: String = lines.next().unwrap().unwrap().trim().to_string();
        let score: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
        students.insert(name, score);
    }
    
    // TODO: Iterate over the hash map and print each student's name and score
    for (key, value) in students {
        println!("{}: {}", key, value);
    }
}