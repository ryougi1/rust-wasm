/*
The process::Output struct represents the output of a finished child process.

The process::Command struct is a process builder.

The std::Child struct represents a running child process and exposes stdin, stdout
and stderr handles for interaction with the underlying process via pipes.


*/
use std::error::Error;
use std::io::prelude::*;
use std::process::{Command, Stdio};

static PANGRAM: &'static str = "the quick brown fox jumped over the lazy dog\n";

pub fn run() {
    println!("\n\n");

    // Construct new command for launching rustc program, add arguments,
    // executes the command as a child process, waiting for it to finish and collecting all of its output,
    // unwraps a result, yielding the content
    let output = Command::new("rustc")
        .arg("--version")
        .output()
        .unwrap_or_else(|e| {
            println!("ERROR: {}", e);
            panic!("failed to execute process: {}", e)
        });

    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout);
        print!("rustc succeeded and stdout was:\n{}", s);
    } else {
        let s = String::from_utf8_lossy(&output.stderr);
        print!("rustc failed and stderr was:\n{}", s);
    }

    // ! Pipes
    let process = match Command::new("wc")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
    {
        Err(why) => panic!("couldn't spawn wc: {}", why.description()),
        Ok(process) => process,
    };

    // Write a string to the `stdin` of `wc`.
    //
    // `stdin` has type `Option<ChildStdin>`, but since we know this instance
    // must have one, we can directly `unwrap` it.
    match process.stdin.unwrap().write_all(PANGRAM.as_bytes()) {
        Err(why) => panic!("couldn't write to wc stdin: {}", why.description()),
        Ok(_) => println!("sent pangram to wc"),
    }

    // Because `stdin` does not live after the above calls, it is `drop`ed,
    // and the pipe is closed.
    //
    // This is very important, otherwise `wc` wouldn't start processing the
    // input we just sent.

    // The `stdout` field also has type `Option<ChildStdout>` so must be unwrapped.
    let mut s = String::new();
    match process.stdout.unwrap().read_to_string(&mut s) {
        Err(why) => panic!("couldn't read wc stdout: {}", why.description()),
        Ok(_) => print!("wc responded with:\n{}", s),
    }
}

// ! Program arguments
/*
Command line arguments can be accessed using std::env::args which
returns an iterator that yields a String for each argument.

Alternatively, 'clap' is a popular crate for command line arguments
https://rust-lang-nursery.github.io/rust-cookbook/cli/arguments.html
*/
