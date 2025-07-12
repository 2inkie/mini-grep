use std::{fs, env};

fn search_in_file(query: &str, file_contents: &str) {
    for line in file_contents.lines() {
        if line.contains(query) {
            println!("Found: {}", line);
        }
    }
}
fn main(){
    let filepath = "./files/".to_string();

    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <query> <filename>", args[0]);
        return;
    }

    let input_args = &args[1..3].to_vec();

    let file = filepath + input_args[1].trim();

    let contents = match fs::read_to_string(&file) {
        Ok(text) => text,

        Err(e) => {
            panic!("Error reading file '{}': {}", file, e);
        }
    };

    let query = input_args[0].trim();
    search_in_file(query, &contents);

}
