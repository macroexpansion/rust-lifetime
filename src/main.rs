// Note: debug expects two parameters with the *same* lifetime
fn debug<'a: 'b, 'b>(a: &'a str, b: &'b str) -> &'b str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}

fn main() {
    let hello = String::from("hello"); // 'hello
    {
        let world = String::from("world");
        let world = &world; // 'world has a shorter lifetime than 'hello
        debug(&hello, world);
    }
}
