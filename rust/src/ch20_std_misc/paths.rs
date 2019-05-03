/*
The Path struct represents file paths in the underlying file system.
posix::Path for UNIX, windows:Path for Windows. 

Path can be created from an OsStr, and provides several methods to get 
information form the file/dir the path points to. 

Note: Path is a vector of bytes Vec<u8>, can can therefore fail when
converting to &str. 
*/

use std::path::Path;

pub fn run() {
    // Create a `Path` from an `&'static str`
    let path = Path::new(".");

    // The `display` method returns a `Show`able structure
    let _display = path.display();

    // `join` merges a path with a byte container using the OS specific
    // separator, and returns the new path
    let new_path = path.join("a").join("b");

    // Convert the path into a string slice
    match new_path.to_str() {
        None => panic!("new path is not a valid UTF-8 sequence"),
        Some(s) => println!("new path is {}", s),
    }
}
