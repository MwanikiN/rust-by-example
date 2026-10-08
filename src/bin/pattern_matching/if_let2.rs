
// Fill in the blank
enum Foo {
    Bar(u8) // tuple variant
}

fn main() {
    let a = Foo::Bar(1);

    if let Foo::Bar(i) = a { // match Foo::Bar
        println!("foobar holds the value: {}", i);

        println!("Success!");
    }
}