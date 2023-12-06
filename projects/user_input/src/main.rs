fn main() {
    // "let"                Declare a new variable
    // "mut"                Make a variable mutable
    // ": String"           Define a variable's type/class
    // "= String::new()"    Create new string as var's value
    let mut user_input: String = String::new();
    
    // "std"                            Rust's Standard library
    // "::io"                           The Input Output module of STD
    // "::stdin()"                      A handler from std::io to handle input
    // ".read_line("                    A method that locks and writes input to a buffer
    // "&mut user_input"                The buffer to be written to when user inputs
    // ").expect("                      Since readline returns a Result, expect that it can fail with a premade error-message
    // "\"Could not grab user input\""  The premade error message
    // ");"                             Close
    std::io::stdin()
        .read_line(&mut user_input)
        .expect("Could not grab user input");


    // "println!("      Macro to print a string to console with a new line 
    // "\"{}\", "       An empty formatable string with an open space to insert one variable
    // "user_input);"   Variable to add to formattable string
    println!("{}", user_input);

}
