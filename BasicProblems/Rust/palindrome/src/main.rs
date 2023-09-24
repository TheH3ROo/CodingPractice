use std::io;

fn main() {
    let input: String = get_input_from_user();
    print!("{}", input);
    let palindrome: bool = is_palindrome(&input);
    println!("{}", palindrome);
    let test_string1 = "racecar";

    if is_palindrome(test_string1) {
        println!("{} is a palindrome.", test_string1);
    } else {
        println!("{} is not a palindrome.", test_string1);
    }
}

fn get_input_from_user() -> String {
    // Create a mutable String to store the user's input
    let mut input = String::new();

    // Prompt the user for input
    println!("Please enter a string:");

    // Read a line from the standard input and store it in the 'input' variable
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Print the user's input
    return input;
}

fn is_palindrome(input: &str) -> bool {
    let trimmed_input = input.trim();
    let reversed = reverse(&trimmed_input);
    return trimmed_input.eq(&reversed);
}

fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}

fn is_palindrome_to_middle(input: String) -> bool {
    for i in 0..input.len()/2 {
        let ch = input.chars().nth(i);
        let ch2 = input.chars().nth(input.len() - i);
        
        println!("ch:{}", ch.unwrap());
        println!("ch2:{}", ch2.unwrap());
        if ch != ch2 { return false; }
    }
    return true;
}