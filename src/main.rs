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
    println!("BMI Calculator\n==============");
    let height: f32 =
        ask_question("What is your height? in METRES please, none of that imperial crap")
            .trim()
            .parse::<f32>()
            .expect("You haven't entered a valid decimal number");
    let weight: f32 = ask_question("What is your weight? In Kilograms!")
        .trim()
        .parse()
        .expect("You haven't entered a valid decimal number");
    let bmi = weight / (height * height);

    match bmi {
        val if val >= 30.0 => println!("According to these numbers, you are obese..."),
        val if val >= 25.0 && val < 30.0 => println!("You are overweight..."),
        val if val >= 18.5 && val < 25.0 => println!("You are normal"),
        _ => println!("You are underweight"),
    };
}
