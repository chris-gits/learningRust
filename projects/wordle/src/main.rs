use rand::seq::{SliceChooseIter, SliceRandom};

mod wordle;

fn main() {

    let mut rng = rand::thread_rng();

    let words_list_main: Vec<wordle::Word> = String::from_utf8(
            std::fs::read("./src/words/main.txt")
            .unwrap())
        .unwrap()
        .split('\n')
        .into_iter()
        .map(|e| wordle::Word::from_string(&String::from(e)))
        .collect();
    let words_list_valid: Vec<wordle::Word> = String::from_utf8(
        std::fs::read("./src/words/valid.txt")
        .unwrap())
    .unwrap()
    .split('\n')
    .into_iter()
    .map(|e| wordle::Word::from_string(&String::from(e)))
    .collect();

    let mut game_active = true;
    while game_active {

        let mut round_active = true;
        let mut round_word: &wordle::Word;
        let mut round_input: String;

        while round_active {

            let round_word = words_list_main.choose(&mut rng).unwrap();

            println!("Current round's word is {}.", round_word.string);
            
            round_input = String::new();
            std::io::stdin().read_line(&mut round_input).unwrap();
            round_input = round_input.trim().into();

            println!("{round_input}");

        }

    }
}

// fn main () {
//     let _words_main_raw = String::from_utf8(
//             std::fs::read("./src/words/main.txt"
//             ).unwrap()
//         ).unwrap();
//     let _words_main = _words_main_raw
//         .split('\n')
//         .into_iter()
//         .map(
//             |x| wordle::Word::from_string(&String::from(x))
//         );
//     let _words_valid_raw = String::from_utf8(
//             std::fs::read("./src/words/valid.txt"
//             ).unwrap()
//         ).unwrap();
//     let _valid_main = _words_valid_raw
//         .split('\n')
//         .into_iter()
//         .map(
//             |x| wordle::Word::from_string(&String::from(x))
//         );

//     for word in _words_main {
//         println!("{}", word.string.to_uppercase())
//     }
// }
