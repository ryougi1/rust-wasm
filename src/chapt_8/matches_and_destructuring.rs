#![allow(dead_code)]
/*
A match block can destructure items in a variety of ways
*/

enum Color {
    // These 3 are specified solely by their name.
    Red,
    Blue,
    Green,
    // These likewise tie `u32` tuples to different names: color models.
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

fn age() -> u32 {
    15
}

pub fn run() {
    // ! Tuples
    let pair = (0, 0);
    match pair {
        (0, y) => println!("First is `0` and `y` is `{:?}`", y),
        (x, 0) => println!("`x` is `{:?}` and last is `0`", x),
        _ => println!("It doesn't matter what they are"),
    }

    // ! enums
    use Color::*;
    let color = HSL(122, 17, 40);
    match color {
        Red => println!("The color is Red!"),
        Blue => println!("The color is Blue!"),
        Green => println!("The color is Green!"),
        RGB(r, g, b) => println!("Red: {}, green: {}, and blue: {}!", r, g, b),
        HSV(h, s, v) => println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
        HSL(h, s, l) => println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
        CMY(c, m, y) => println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
        CMYK(c, m, y, k) => println!(
            "Cyan: {}, magenta: {}, yellow: {}, key (black): {}!",
            c, m, y, k
        ),
        // Don't need another arm because all variants have been examined
    }

    // ! pointers/ref
    /*
    For pointers, distinction between destructuring and dereferencing.
    Dereferencing uses *
    Destructuring uses &, ref, ref mut
    */

    // Assign a reference of type `i32`. The `&` signifies there
    // is a reference being assigned.
    let reference = &4;
    // same as let ref reference = 4

    match reference {
        // If `reference`s is pattern matched against `&val`, it results
        // in a comparison like:
        // `&i32`
        // `&val`
        // ^ We see that if the matching `&`s are dropped, then the `i32`
        // should be assigned to `val`.
        &val => println!("Got a value via destructuring: {:?}", val),
    }

    // To avoid the `&`, you dereference before matching.
    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }

    // Accordingly, by defining 2 values without references, references
    // can be retrieved via `ref` and `ref mut`.
    let value = 5;
    let mut mut_value = 6;

    // Use `ref` keyword to create a reference.
    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    // Use `ref mut` similarly.
    match mut_value {
        ref mut m => {
            // Got a reference. Gotta dereference it before we can
            // add anything to it.
            *m += 10;
            println!("We added 10. `mut_value`: {:?}", m);
        }
    }

    // ! structs
    struct Foo {
        x: (u32, u32),
        y: u32,
    }

    let foo = Foo { x: (1, 2), y: 3 };
    let Foo { x: (a, b), y } = foo;
    println!("a = {}, b = {},  y = {} ", a, b, y);

    // Order is not important
    let Foo { y: i, x: j } = foo;
    println!("i = {:?}, j = {:?}", i, j);

    // and you can also ignore some variables:
    let Foo { y, .. } = foo;
    println!("y = {}", y);

    // ! guards
    let pair = (-2, -2);

    println!("Tell me about {:?}", pair);
    match pair {
        (x, y) if x == y => println!("These are twins"),
        // The ^ `if condition` part is a guard
        (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
        (x, _) if x % 2 == 1 => println!("The first one is odd"),
        _ => println!("No correlation..."),
    }

    // ! binding
    match age() {
        0 => println!("I'm not born yet I guess"),
        // Could `match` 1 ... 12 directly but then what age
        // would the child be? Instead, bind to `n` for the
        // sequence of 1 .. 12. Now the age can be reported.
        n @ 1...12 => println!("I'm a child of age {:?}", n),
        n @ 13...19 => println!("I'm a teen of age {:?}", n),
        // Nothing bound. Return the result.
        n => println!("I'm an old person of age {:?}", n),
    }
}
