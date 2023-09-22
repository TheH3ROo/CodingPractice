use std::io;

fn main() {
    println!("Hello, world!");
    fizz_buzz_to_n(30);
    println!("");
    fizz_buzz_concat_to_n(30);
    io::stdin().read_line(&mut String::new()).unwrap();
}

fn fizz_buzz_to_n(number: i32){
    for n in 1..=number  {
        if n % 3 == 0 && n % 5 == 0 {
            println!("FizzBuzz")
        } else if n % 3 == 0 {
            println!("Fizz")
        } else if n % 5 == 0 {
            println!("Buzz")
        } else{
            println!("{}", n);
        }
    }
}

fn fizz_buzz_concat_to_n(number: i32){
    let mut result;
    for n in 1..=number  {
        result = String::new();
        if n % 3 == 0 {
            result.push_str("Fizz");
        } 
        if n % 5 == 0 {
            result.push_str("Buzz");
        } 
        if result.is_empty(){
            result.push_str(&n.to_string());
        }
        println!("{}", result);
    }
}