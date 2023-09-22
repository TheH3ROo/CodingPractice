use std::io::{stdout, BufWriter};

use ferris_says::say;

fn main() {
    println!("Hello, world!");
    ferris_says_to_console(String::from("Hello fellow Rustaceans!"));
}

fn ferris_says_to_console(message: String){
    let stdout = stdout();
    let message = String::from(message);
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();
}
