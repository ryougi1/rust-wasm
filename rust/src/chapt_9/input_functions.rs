#![allow(dead_code)]

/*
Functions can also be used as arguments to functions. A function that takes
a closure as parameter can also take any function that satisfies the trait
bound of that closure.
*/

// Define a function which takes a generic F argument
// bounded by Fn and calls it.
fn call_me<F: Fn()>(f: F) {
    f();
}

// Define a wrapper function that satisfies the Fn bound
fn a_function() {
    println!("I'm a useless function");
}

pub fn run() {
    let a_closure = || println!("I'm a useless closure");

    call_me(a_closure);
    call_me(a_function);
}
