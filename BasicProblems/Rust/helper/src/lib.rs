use std::io;

pub fn get_string_from_user() -> String {
    // Create a mutable String to store the user's input
    let mut input = String::new();

    // Prompt the user for input
    println!("Please enter a string:");

    // Read a line from the standard input and store it in the 'input' variable
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Print the user's input
    return input;
}

pub fn get_number_from_user() -> i32 {
    // Create a mutable String to store the user's input
    let mut input = String::new();

    // Prompt the user for input
    println!("Please enter a number:");

    // Read a line from the standard input and store it in the 'input' variable
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Print the user's input
    return input.trim().parse().expect("Invalid input");
}

pub trait ReverseStringExt {
    fn reverse(&self) -> String;
}

impl ReverseStringExt for String {
    fn reverse(&self) -> String {
        self.chars().rev().collect()
    }
}

impl ReverseStringExt for &str {
    fn reverse(&self) -> String {
        self.chars().rev().collect()
    }
}