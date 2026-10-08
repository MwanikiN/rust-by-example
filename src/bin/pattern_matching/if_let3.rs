#[allow(dead_code)]
enum Foo {
    Bar,
    Baz,
    Qux(u32)
}

fn main() {
    let a = Foo::Qux(10);

    // Remove the codes below, using `match` instead 
    // if let Foo::Bar = a {
    //     println!("match foo::bar")
    // } else if let Foo::Baz = a {
    //     println!("match foo::baz")
    // } else {
    //     println!("match others")
    // }
    match a {
        Foo::Bar => println!("match foo::bar"),
        Foo::Baz => println!("match foo::baz"),
        _ => println!("match others")
    }
}