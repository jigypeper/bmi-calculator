use std::{fmt::Debug, io, str::FromStr};

fn ask_question<T>(question: &str) -> T
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    loop {
        println!("{question}");
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim().parse::<T>() {
            Ok(num) => return num,
            Err(e) => println!("{:?}, please try again.", e),
        }
    }
}

fn main() {
    println!("BMI Calculator\n==============");
    let height: f32 =
        ask_question("What is your height? in METRES please, none of that imperial crap");
    let weight: f32 = ask_question("What is your weight? In Kilograms!");
    let bmi = weight / (height * height);

    match bmi {
        val if val >= 30.0 => {
            println!(
                "BMI: {:.2}, according to these numbers, you are obese...",
                bmi
            )
        }
        val if val >= 25.0 && val < 30.0 => println!("BMI: {:.2}, you are overweight...", bmi),
        val if val >= 18.5 && val < 25.0 => println!("BMI: {:.2}, you are normal", bmi),
        _ => println!("BMI: {:.2}, you are underweight", bmi),
    };
}
