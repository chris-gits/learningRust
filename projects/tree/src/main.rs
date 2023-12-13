use colored::*;

fn main () {
    let args: Vec<String> = std::env::args().collect();

    // Path
    let default_path = String::from(".");
    let path = std::path::Path::new(match args.get(1) {
        Some(arg) => arg,
        None => &default_path,
    });

    // Path Validity
    if !path.exists() {println!("{}", "Selected path does not exist.".red()); return}
    if !path.is_dir() {println!("{}", "Selected path is not a directory.".red()); return}
    
    // Iterate Directory
    fn iterDir(path: std::path::PathBuf, depth: usize) {
        let mut read_dir = match std::fs::read_dir(path) {
            Ok(valid_dir) => valid_dir,
            Err(error) => {return}
        };
        let mut curr_item: std::path::PathBuf;
        for item_result in read_dir {
            curr_item = item_result.unwrap().path();
            println!("{}", "|  ".repeat(depth) + "└──" + curr_item.file_name().unwrap().to_str().unwrap());
            if curr_item.is_dir() {
                iterDir(curr_item.to_path_buf(), depth+1)
            }
        }
    }

    iterDir(path.to_path_buf(), 0);

    



}