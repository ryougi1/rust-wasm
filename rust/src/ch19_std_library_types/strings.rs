/*
Two types of strings in Rust: String and &str

A String is stored as a vector of bytes vec<u8>, but guaranteed to
always be a valid UTF-8 sequence. They are heap allocated, growable and
not null terminated.

&str is a slice &[u8] that always points to a valid UTF-8 sequence,
and can be used to view into a String, just like &[T] is a view into Vec<T>.
*/

pub fn run() {
    println!("\n\n");
    // A reference to a string allocated in read only memory
    let pangram: &'static str = "the quick brown fox jumps over the lazy dog";
    println!("Pangram: {}", pangram);

    // Iterate over words in reverse, no new string is allocated
    println!("Words in reverse");
    for word in pangram.split_whitespace().rev() {
        println!("> {}", word);
    }

    // Copy chars into a vector, sort and remove duplicates
    let mut chars: Vec<char> = pangram.chars().collect();
    chars.sort();
    chars.dedup();

    // Create an empty and growable `String`
    let mut string = String::new();
    for c in chars {
        // Insert a char at the end of string
        string.push(c);
        // Insert a string at the end of string
        string.push_str(", ");
    }
    println!("Chars: {}", string);
}