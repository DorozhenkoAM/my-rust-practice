use std::io;

fn is_prime(n: &u32) -> bool {
    if *n < 2 {
        return false;
    }

    let mut i = 2;
    while i * i <= *n {
        if *n % i == 0 {
            return false;
        }
        i += 1;
    }

    true
}

fn main() {
    println!("Enter a number to check if it is prime:"); //зазвичай писав подібні речі на українській мові, але в прикладі була англійська, тому так...

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim();

    match input.parse::<u32>() {
        Ok(number) => {
            if is_prime(&number) {
                println!("{} is a prime number.", number);
            } else {
                println!("{} is not a prime number.", number);
            }
        }
        Err(_) => println!("Please enter a valid integer."),
    }
}