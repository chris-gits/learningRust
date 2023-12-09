use std::slice::SliceIndex;

mod wordle;

fn main() {
    let test_word_1 = wordle::Word::from_string(&String::from("asd"));
    let test_word_2 = wordle::Word::from_string(&String::from("bsa"));
    
    let cmp_res = test_word_1.compare_to(&test_word_2);

    for (i, c) in cmp_res.into_iter().enumerate() {
        println!("{} => {} : {}",
            test_word_1.string.get(i).unwrap()
        )
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
