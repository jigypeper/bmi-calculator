use std::io;

// TODO: Make this function able to output other data types?
// or just parse strings to other types in main fn
fn ask_question(question: &str) -> String {
    println!("{question}");
    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Oops! Looks like there be a sea monster in the I/O waters.");

    input.trim().to_string()
}

fn main() {
    println!("Hello, world!");
}
