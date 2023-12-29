# Crafting User Interactions and Parsing Data in Rust: 
# A Dive into a Simple BMI Calculator

In the realm of software development, user interaction and data handling are pivotal. 
Rust, known for its safety and performance, offers an intriguing approach to handle 
user inputs and data parsing. Let's explore these concepts through a simple BMI 
(Body Mass Index) Calculator application.

## Introduction to the BMI Calculator in Rust

Our journey begins with a basic Rust program designed to calculate a user's BMI. 
The BMI is a simple calculation using a person's height and weight. While the concept 
is straightforward, the implementation gives us a perfect playground to discuss user 
interaction and data parsing in Rust.

The program comprises two primary functions:

1. ask_question: To interact with the user and get their input.
2. main: To drive the application, perform calculations, and provide results.

### User Interaction with ask_question

The ask_question function is our primary means of interacting with the user. It prints 
a question to the console, waits for the user's input, trims it, and returns it as a String. 
This method showcases basic user interaction in Rust using the std::io library. If you have
not read my previous blog post where we first implement this reasoning, you can find it here.

```rust
fn ask_question(question: &str) -> String {
    println!("{question}");
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Oops! Failed to read input.");
    input.trim().to_string()
}

```
### Parsing User Input in the Main Function

In the main function, we see the application's core: prompting for height and weight, parsing these 
values to floating-point numbers, and then calculating the BMI based on these inputs. This process 
illustrates Rust's handling of string parsing and error management.

```rust
let height: f32 = ask_question("Enter your height in meters: ")
    .trim()
    .parse::<f32>()
    .expect("Please enter a valid decimal number for height.");

let weight: f32 = ask_question("Enter your weight in kilograms: ")
    .trim()
    .parse::<f32>()
    .expect("Please enter a valid decimal number for weight.");

let bmi = weight / (height * height);

```

Each parse call attempts to convert the string input into a floating-point number, a common operation 
when dealing with numerical user input. The expect method provides a simple way to crash the program 
with a friendly error message if the parsing fails, indicating the input wasn't a valid number.

Once we have our number, we can use the match statement in rust to display an output to the user.
Think of a match statement as a powerful control flow construct that allows you to compare a value against 
a series of patterns and execute code based on which pattern matches. It's similar to the switch case 
statement found in other languages like C or Java but much more powerful due to Rust's pattern matching 
capabilities. Although, in the case below, an if statement would work just as well.

```rust
match bmi {
    val if val >= 30.0 => println!("According to these numbers, you are obese..."),
    val if val >= 25.0 && val < 30.0 => println!("You are overweight..."),
    val if val >= 18.5 && val < 25.0 => println!("You are normal"),
    _ => println!("You are underweight"),
};

```
