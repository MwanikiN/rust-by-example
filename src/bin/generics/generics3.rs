// mod stack;

use rust_by_example::lib::stack::Stack;

fn main() {
    // Read three integers from input
    let mut input1 = String::new();
    std::io::stdin().read_line(&mut input1).expect("Failed to read line");
    let num1: i32 = input1.trim().parse().expect("Invalid number");

    let mut input2 = String::new();
    std::io::stdin().read_line(&mut input2).expect("Failed to read line");
    let num2: i32 = input2.trim().parse().expect("Invalid number");

    let mut input3 = String::new();
    std::io::stdin().read_line(&mut input3).expect("Failed to read line");
    let num3: i32 = input3.trim().parse().expect("Invalid number");

    // TODO: Create a new Stack
    let mut new_stack: Stack<i32> = Stack::new();
    // TODO: Push the three numbers onto the stack (num1, num2, num3 in that order)
    for i in 0..3 {
        new_stack.push(match i {
            0 => num1,
            1 => num2,
            2 => num3,
            _ => unreachable!(),
        });
    }
   
    for _ in 0..4 {
        println!("{}", new_stack.pop().unwrap_or(-1));
    }

    // Use unwrap_or(-1) to handle the empty stack case
}
