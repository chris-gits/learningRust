use std::io::Write;
use rand::Rng;

const GUESS_RANGE_MIN: i64 = 0;
const GUESS_RANGE_MAX: i64 = 10;

fn main() {

    let mut to_guess_number: i64 = 0;
    let mut refresh_number = true;

    loop {
        // Game Details
        println!("Guess the number! (Between {GUESS_RANGE_MIN} and {GUESS_RANGE_MAX})");
        if refresh_number {
            to_guess_number = rand::thread_rng().gen_range(GUESS_RANGE_MIN..=GUESS_RANGE_MAX);
            refresh_number = false
        }

        // Accept User Input
        print!("Enter your guess: ");
        std::io::stdout()
        .flush()
        .err();
        let mut user_guess = String::new();
        std::io::stdin()
            .read_line(&mut user_guess)
            .expect("Failed to read line");
        
        // Format User Input
        let user_guess_formatted: i64;
        match user_guess.trim().parse::<i64>() {
            Err(_error) => {
                println!("Not a valid number.");
                continue;
            }
            Ok(guess) => {
                user_guess_formatted = guess;
            },
        };

        // Test Guess to Answer & Finish
        if user_guess_formatted == to_guess_number {
            println!("You have guessed the number!");
            refresh_number = true
        } else {
            println!("You have not guessed the number!");
        }

    }
}