#![allow(dead_code)]
/*
Can choose how to capture variables on the fly mostly without type annotation
as seen before. But not when taking a closure as an input parameter to a
function. The closure's complete type must be annotated using one of
the following traits:

Fn: the clasure captures by reference &T
FnMut: the closure captures by mutable reference &mut T
FnOnce: the closure captures by value T
*/

// A function hich takes a closure as an argument and calls it
// fn apply<F>(f: F)
// where
//     F: FnOnce(),
// {
//     f();
// }

fn apply<F: FnOnce()>(f: F) {
    f();
}

// A function which takes a closure and returns an 'i32'
// fn apply_to_3<F>(f: F) -> i32
// where
//     F: Fn(i32) -> i32,
// {
//     f(3)
// }
fn apply_to_3<F: Fn(i32) -> i32>(f: F) -> i32 {
    f(3)
}

pub fn run() {
    use std::mem;

    let greeting = "hello";
    let mut farewell = "goodbye".to_owned();

    // Capture 2 variables: 'greeting' by reference and
    // 'farewell' by value
    let diary = || {
        // 'greeting' is by reference: requires 'Fn'
        println!("{} my friend.", greeting);

        // Mutation forces 'farewell' to be captured by
        // mutable reference. Requires 'FnMut'.
        farewell.push_str("!!");
        println!("{} my friend", farewell);

        // Manually calling drop forces 'farewell' to be
        // captured by value. Now requires 'FnOnce'.
        mem::drop(farewell);
    };

    apply(diary);

    let double = |x| 2 * x;
    println!("Double of 3: {}", apply_to_3(double));
}
