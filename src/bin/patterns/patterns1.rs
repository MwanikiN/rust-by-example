/* The @ operator lets us create a variable that holds a value,
 at the same time we are testing that value to see whether it matches a pattern.
 @ (at) operator in Rust is used for subpattern binding.
 */


struct Point {
    x: i32,
    y: i32,
}

fn main() {
    // Fill in the blank to let p match the second arm
    let p = Point { x: 5, y: 10 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        // Second arm
        Point { x: 0..=5, y: y@ (10 | 20 | 30) } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}