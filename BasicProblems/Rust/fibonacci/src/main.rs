fn main() {
    println!("Hello, world!");
    println!("This is my fibonacci project!");
    println!("The program will tell you the 'Nth' fibonacci number by your input");

    let input: i32 = helper::get_number_from_user();

    println!("{}. fibonnaci number is: {}", input, nth_fibonacci(input));
}

fn nth_fibonacci(input: i32) -> i32 {
    if input == 0 { return 0; }
    if input == 1 { return 1; }
    nth_fibonacci(input - 1) + nth_fibonacci(input - 2)
}
