#![allow(dead_code)]
/*
Closures, also called lambda expression or lambdas,
are functions that can capture the enclosing environment.

The syntax and capabilities of closures make them very convinient for
on the fly usage. Calling a closure is exactly like calling a function.
However, both input and return types can be inferred and input variabl
names must be specified.
*/

pub fn run() {
    // Increment via closures and functions
    fn function(i: i32) -> i32 {
        i + 1
    }

    // Closures are anonymous, here we are binding them to references
    // Annotation is identical to function annotation but is optional
    // as are the `{}` wrapping the body. These nameless functions
    // are assigned to appropriately named variables.
    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred = |i| i + 1;

    let i = 1;
    // Call the function and closures
    println!("function: {}", function(i));
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred: {}", closure_inferred(i));

    // A closure taking no arguments which returns an 'i32'
    let one = || 1;
    println!("closure returning one: {}", one());

    // ! Capturing
    /*
    Closures can capture variables by reference &T, by mutable reference &mut T,
    and by value T. They preferentially capture variables by reference.
    */

    use std::mem;

    let color = "green";

    // Closure borrows &color and stores the borrow and closure
    // in the 'print' variable. It remains borrowed until 'print'
    // goes out of scope. 'println' only requires 'by reference' so
    // so it doesn't impose anything more restrictive
    let print = || println!("color: {}", color);
    print();
    print();

    // A closure to increment count could take either '&mut count'
    // or 'count' but '&mut count' is less restrictive so it takes that.
    // Immediately borrows 'count'.
    // A 'mut' is required on 'inc' because a '&mut' is stored inside.
    let mut count = 0;
    let mut inc = || {
        count += 1;
        println!("count: {}", count);
    };
    inc();
    inc();

    // mem::drop requires T so this must take by value. A copy type
    // would copy into the closure leaving the original untouched.
    // A non-copy must move and so 'movable' immediately moves into
    // the closure.
    let movable = Box::new(3);
    let consume = || {
        println!("movable: {:?}", movable);
        mem::drop(movable);
    };
    consume();
    // consume(); not possible now

    // Can use 'move' before vertical pipes to force closure
    // taking ownership of captured variables
    let haystack = vec![1, 2, 3];
    let contains = move |needle| haystack.contains(needle);
    println!("{}", contains(&1));
    println!("{}", contains(&4));
}
