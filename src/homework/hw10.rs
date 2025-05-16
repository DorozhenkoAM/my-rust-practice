use std::io;

fn is_palindrome(x: u32) -> bool {
    let s = x.to_string();
    let reversed: String = s.chars().rev().collect();
    s == reversed
}

fn main() {
    println!("Enter a number to check if it is a palindrome:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim();

    match input.parse::<u32>() {
        Ok(number) => {
            if is_palindrome(number) {
                println!("{} is a palindrome.", number);
            } else {
                println!("{} is not a palindrome.", number);
            }
        }
        Err(_) => println!("Please enter a valid number."), //якщо щось не по плану то це такий план.
    }
}