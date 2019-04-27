use std::fs;

pub fn run() {
    let contents = fs::read_to_string("src/texts/Dickinson.txt")
        .expect("Something went wrong reading the file");

    // println!("{}", contents);
    let mut result = (0, "");

    // Imperative:
    for word in contents.split_whitespace() {
        // println!("{}", word);
        if word.len() > result.0 {
            result.0 = word.len();
            result.1 = word;
        }
    }

    println!(
        "Longest word was '{}' and has {} characters",
        result.1, result.0
    );
}
