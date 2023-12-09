mod wordle;

fn main() {
    let test_word_guess = wordle::Word::from_string(&String::from("tiers"));
    let test_word_answer = wordle::Word::from_string(&String::from("start"));
    
    let comp_res = test_word_guess.compare_to(&test_word_answer);
    match comp_res {
        Ok(comp_vec) => {
            for comp in comp_vec {
                println!("{:?}", comp)
            }
        },
        Err(error) => {
            if error == wordle::ComparisonError::MismatchedLength {
                println!("Words' lengths do not match.")
            } else {
                println!("Errored with \"{:?}\"", error)
            }
        },
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
