use std::fs;

fn main() {

    // Read file into String
    let mut funny_string = String::from_utf8(
        fs::read(
            "./src/input.txt"
        ).unwrap()
    ).unwrap();

    // Mess with String
    funny_string = funny_string.to_uppercase();

    // Output String into file
    fs::write(
        "./src/output.txt",
        funny_string.as_bytes()
    ).unwrap();

    // fs::write(
    //     "./src/output.txt",
    //     String::from_utf8(
    //         fs::read("./src/input.txt").unwrap()
    //         )
    //         .unwrap()
    //         .to_uppercase()
    //     )
    // .unwrap()
    

}