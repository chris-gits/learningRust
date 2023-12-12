use rand::seq::SliceRandom;
use colored::*;

mod wordle;

const MAX_GUESSES: u32 = 6;
fn main() {

    let mut rng = rand::thread_rng();

    let words_list_main: Vec<wordle::Word> = String::from_utf8(
    std::fs::read("./src/words/main.txt")
        .unwrap())
    .unwrap()
    .split('\n')
    .into_iter()
    .map(|e| wordle::Word::from_string(&String::from(e.trim())))
    .collect();
    let words_list_valid: Vec<wordle::Word> = String::from_utf8(
    std::fs::read("./src/words/valid.txt")
        .unwrap())
    .unwrap()
    .split('\n')
    .into_iter()
    .map(|e| wordle::Word::from_string(&String::from(e.trim())))
    .collect();

    let mut game_active = true;

    let mut round_active = true;
    let mut round_word = &wordle::Word::from_string(&String::new());
    let mut round_number: u32 = 0;
    let mut round_guesses: u32 = 0;
    let mut round_comparison = Vec::<wordle::Comparison>::new();

    let mut input_active = true;
    let mut input_string = String::new();
    let mut input_word = wordle::Word::from_string(&String::new());
    let mut input_valid = false;
    let mut input_string_chars: Vec<char>;

    while game_active {

        round_word = words_list_main.choose(&mut rng).unwrap();
        round_number += 1;
        round_guesses = 0;

        println!("{} {} {} {}",
        "~~~".italic(),
        "Round".bold(),
        round_number.to_string().green().bold(),
        "~~~".italic()
        );

        round_active = true;
        while round_active {

            input_active = true; 
            while input_active {

                print!("Guess a word {} -> ",
                    format!("(Attempt {}/{})", round_guesses+1, MAX_GUESSES).underline());
                std::io::Write::flush(&mut std::io::stdout()).unwrap();
                input_string = String::new();
                std::io::stdin().read_line(&mut input_string).unwrap();
                input_string = input_string.trim().into();
                input_word = wordle::Word::from_string(&input_string);

                if !words_list_valid.contains(&input_word) {
                    println!("{}", "That word is not valid.".red());
                    continue;
                }

                input_valid = true;
                input_active = false;

            }

            round_comparison = input_word.compare_to(&round_word);

            input_string_chars = input_string.chars().collect();
            for char_i in 0..input_string_chars.len() {
                if char_i > round_comparison.len() {
                    print!("{}", input_string_chars[char_i].to_string())
                } else {
                    match round_comparison[char_i] {
                        wordle::Comparison::Valid => {print!("{}", input_string_chars[char_i].to_string().on_green())},
                        wordle::Comparison::Exists => {print!("{}", input_string_chars[char_i].to_string().on_yellow())},
                        wordle::Comparison::Invalid => {print!("{}", input_string_chars[char_i].to_string())},
                    }
                }
            }
            print!("\n");
            std::io::Write::flush(&mut std::io::stdout()).unwrap();

            if round_word.string == input_string {
                println!("{} The word was: {}.", "You have won this round!".green(), round_word.string.blue());
                round_active = false;
                continue;
            } else {
                round_guesses += 1;
                if round_guesses >= MAX_GUESSES {
                    println!("{} The word was: {}.", "You have lost this round.".red(), round_word.string.blue());
                    round_active = false
                }
            }
        }
    }
}