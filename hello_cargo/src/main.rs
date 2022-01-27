use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=10);
    let mut counter: i32 = 0;

    let result = 'counting_up: loop {
        let mut guess: String = String::new();
        counter += 1;

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read this line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // The scope in which the variable s is valid is the same as any function
        // parameter’s scope, but the value pointed to by the reference is not
        // dropped when s stops being used because s doesn’t have ownership.
        // When functions have references as parameters instead of the actual values,
        // we won’t need to return the values in order to give back ownership, because we never had ownership.
        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("too low"),
            std::cmp::Ordering::Greater => println!("too high"),
            std::cmp::Ordering::Equal => {
                println!("You guesed!");
                break 'counting_up counter;
            }
        }
    };

    print_stats(result);
}

fn print_stats(x: i32) -> i32 {
    println!("You played the game! {}x", x);
    return x;
}
