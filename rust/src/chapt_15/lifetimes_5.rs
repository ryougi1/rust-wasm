/*
Some lifetime patterns are overwhelmingly common and so 
the borrow checker will implicitly add them to save typing 
and to improve readability. This process of implicit addition 
is called elision. Elision exists in Rust solely because these 
patterns are common.

The following code shows a few examples of elision.
*/

// `elided_input` and `annotated_input` essentially have identical signatures
// because the lifetime of `elided_input` is elided by the compiler:
fn elided_input(x: &i32) {
    println!("`elided_input`: {}", x);
}

fn annotated_input<'a>(x: &'a i32) {
    println!("`annotated_input`: {}", x);
}

// Similarly, `elided_pass` and `annotated_pass` have identical signatures
// because the lifetime is added implicitly to `elided_pass`:
fn elided_pass(x: &i32) -> &i32 { x }

fn annotated_pass<'a>(x: &'a i32) -> &'a i32 { x }
