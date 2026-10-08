
fn main() {
    match_number(1);
    match_number(2);
    match_number(3);
    match_number(4);
    match_number(5);
    match_number(6);
    match_number(7);
    match_number(8);
    match_number(9);
    match_number(10);
    match_number(-1);
    match_number(-1000);
    match_number(11);
    match_number(1000);
}
fn match_number(n: i32) {
    match n {
        // Match a single value
        1 => println!("One!"),
        // Fill in the blank with `|`, DON'T use `..` or `..=`
        2 | 3 | 4 | 5 => println!("match 2 -> 5"),
        // Match an inclusive range
        6..=10 => {
            println!("match 6 -> 10")
        },
        _ => {
            println!("match -infinite -> 0 or 11 -> +infinite")
        }
    }
}