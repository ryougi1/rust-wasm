#![allow(dead_code)]

/*
The enum keyword allows the creation of a type which may be one of a few different variants.
Any variant which is valid as a struct is also valid as an enum, meaning regular struct,
tuple struct, unit struct.
*/

// Create an `enum` to classify a web event. Note how both
// names and type information together specify the variant:
// `PageLoad != PageUnload` and `KeyPress(char) != Paste(String)`.
// Each is different and independent.
enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

// A function which takes a `WebEvent` enum as an argument and
// returns nothing.

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        // Destructure `c` from inside the `enum`.
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        // Destructure `Click` into `x` and `y`.
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        }
    }
}

// ! use
// The use declaration can be used so manual scoping isn't needed
enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

// ! c-like
// enum can also be used as C-like enums
// enum with implimict discriminator (starts at 0)
enum Number {
    Zero,
    One,
    Two,
}

// enum with explicit discriminator
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

pub fn run() {
    let pressed = WebEvent::KeyPress('X');
    // `to_owned()` creates an owned `String` from a string slice.
    // let pasted = WebEvent::Paste("my text".to_owned());
    let pasted = WebEvent::Paste(String::from(
        "a';DROP TABLE users; SELECT * FROM userinfo WHERE 't' = 't",
    ));
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    // Explicitlly 'use' each name so they are available without manual scoping
    use Status::{Poor, Rich};
    // use Work::*;

    // Equivalent to Status::Poor and Work::Civilian
    let status = Poor;
    // let work = Civilian;
    match status {
        Rich => println!("$$$$$$$$$$"),
        Poor => println!("$$"),
    }

    use Color::*;
    use Number::*;
    println!("Zero is {}", Zero as i32);
    println!("Roses are #{:06x}", Red as i32);
    println!("Violets are #{:06x}", Blue as i32);
}
