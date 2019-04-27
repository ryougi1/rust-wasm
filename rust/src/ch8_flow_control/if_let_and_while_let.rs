#![allow(unused)]
/*
Type Option represents an optional value: either Some which contains a value,
or None which does not.

For some use cases, when matching enums, match is awkward.
Consider an optional setting that is to match. First check if its Some(i),
then filter for i, including _ for exhaustive matching.
if let is cleaner and allows for various failure options to be specified
*/

enum Foo {
    Bar,
    Baz,
    Qux(u32),
}

pub fn run() {
    // All have type 'Option<i32>
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    // If 'number' destructure into 'Some(i),
    // then evaluate the block ('{}'), has access to i
    if let Some(i) = number {
        println!("Macthed {:?}", i);
    }

    if let Some(i) = letter {
        println!("Matched {:?}", i);
    } else {
        println!("Didn't match");
    }

    // In the same way, if let can be used to match any enum value
    use Foo::*;
    let a = Bar;
    let b = Baz;
    let c = Qux(100);

    if let Bar = a {
        println!("a is foobar");
    }

    if let Bar = b {
        println!("will not print");
    }

    if let Qux(value) = c {
        println!("Value of c is: {}", value);
    }

    /*
    Similar to if let, while let can make awkward match sequences more tolerable.
    Instead of a loop with a match inside and if else inside the match:
    */

    let mut optional = Some(0);

    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit");
            optional = None;
        } else {
            println!("i is {:?}. Try again.", i);
            optional = Some(i + 1);
        }
    }
}
