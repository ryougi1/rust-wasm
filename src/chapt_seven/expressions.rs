#![allow(dead_code)]
#![allow(unused)]
/*
A rust program is mostly made up a series of statements.
There are a few kinds of statements in Rust. The most common two are:
1. declaring a variable binding: let x 5;
2. using a ; with an expression: x + 1;
*/

pub fn run() {
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // This expression will be assigned to 'y'
        x_cube + x_squared + x
    };

    let z = {
        // The ; supresses this expression and '()' is assigned to 'z'
        2 * x;
    };

    println!("x is {}", x);
    println!("y is {}", y);
    println!("z is {:?}", z);
}
