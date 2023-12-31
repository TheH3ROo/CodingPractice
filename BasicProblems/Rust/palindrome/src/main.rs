use helper::ReverseStringExt;

fn main() {
    let input: String = helper::get_string_from_user();
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

fn is_palindrome(input: &str) -> bool {
    let trimmed_input = input.trim();
    let reversed = trimmed_input.reverse();
    return trimmed_input.eq(&reversed);
}

//TODO: Finish this fn
// fn is_palindrome_to_middle(input: String) -> bool {
//     for i in 0..input.len()/2 {
//         let ch = input.chars().nth(i);
//         let ch2 = input.chars().nth(input.len() - i);
        
//         println!("ch:{}", ch.unwrap());
//         println!("ch2:{}", ch2.unwrap());
//         if ch != ch2 { return false; }
//     }
//     return true;
// }