# 11:23 7 Dec 2023 
WHAT THE ACTUAL FUCK IS THIS CODE????
```rust
fn main () {
    let _words_main: Vec<&str> = String::from_utf8(
            std::fs::read("./src/words/main.txt"
            ).unwrap()
        ).unwrap()
        .split('\n')
        .collect();
    let _words_valid: Vec<&str> = String::from_utf8(
            std::fs::read("./src/words/valid.txt"
            ).unwrap()
        ).unwrap()
        .split('\n')
        .collect(); 
}
```

LIKE IT MAKES SENSE BUT ONE LINE TO: READ A FILE -> UNWRAP ITS BYTES -> CONVERT THOSE TO A STRING -> UNWRAP THE STRING -> SPLIT IT BY NEW LINES -> CONVERT IT TO VECTOR

LIKE HUHHH??? 

And I still need to make all of them into my Word class x-x

# 15:20 9 Dec 2023 
"Rust makes you feel like a genius" stfu rust makes me feel dumb as hell.

I got the word comparison function working FINALLY but I think I might just wrap its output in a Result. Panicking because the word length doesn't match is kind of overkill and even if I don't plan on reusing this code I might as well learn how Results work. 

ILL TRY TO REMEMBER TO UPDATE THIS RECORD I KEEP FORGETTING 
