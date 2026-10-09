use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the comma-separated tasks
    let tasks_input = lines.next().unwrap().unwrap();
    
    // Read the task number to remove (1-based indexing)
    let task_to_remove = lines.next().unwrap().unwrap();
    let task_number: usize = task_to_remove.trim().parse().unwrap();
    
    // Read the new task to add
    let new_task = lines.next().unwrap().unwrap();
    
    // TODO: Write your code below
    // Split the tasks_input by commas and create a mutable vector
    let mut todo_list: Vec<&str> = tasks_input.split(',').map(|task| task.trim()).collect();
    // Remove the task at the specified index (convert 1-based to 0-based)
    let task_number: usize = task_number - 1_usize;
    todo_list.remove(task_number);
    // Add the new task to the vector
    todo_list.push(&new_task);
    // Output the results
    // Print total tasks in format: Total tasks: X
    println!("Total tasks: {}", todo_list.len());
    // Print each task in format: Task: [task description]
    for task in todo_list {
        println!("Task: {}", task);
    }
}