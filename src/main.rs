use serde::Deserialize;

#[derive(Deserialize)]
#[allow(dead_code)]
struct ParsedData<'a> {
    name: &'a str,
    some_other_value: String,
}

#[derive(Debug)]
struct HasName<'a> {
    name: &'a str,
}

fn extract_name_success<'a>(data: &ParsedData<'a>) -> HasName<'a> {
    HasName { name: data.name }
}

fn extract_name_fail<'a>(data: &'a ParsedData<'a>) -> HasName<'a> {
    HasName { name: data.name }
}

// example code from here: https://users.rust-lang.org/t/lifetime-issues-with-async-trait/34111
fn run_extract_name() {
    let data = r#"{ "name": "test", "some_other_value": "hello" }"#;

    // Parse the above json. Parsed will contain slices into data.
    let parsed = serde_json::from_str(data).unwrap();

    // The name value now contains a value it took from parsed.
    let name = extract_name_success(&parsed);

    // this will fail to compile
    // let name = extract_name_fail(&parsed);

    // Now we drop parsed.
    drop(parsed);

    // However we can stil use name, because it's really borrowing from data, not parsed.
    println!("{:?}", name);
}

fn main() {
    run_extract_name();

    let hello = String::from("hello"); // 'hello
    {
        let world = String::from("world");
        let world = &world; // 'world has a shorter lifetime than 'hello
        debug(&hello, world);
    }
}

// Note: debug expects two parameters with the *same* lifetime
fn debug<'a: 'b, 'b>(a: &'a str, b: &'b str) -> &'b str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}
