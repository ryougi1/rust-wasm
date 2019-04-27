#![allow(dead_code)]

/*
Closures as output parameters are also possible. Currently, only supports
returning concrete (non-generic) types. This can be done via boxing.

Valid traits for returns are:
Fn: normal
FnMut: normal
FnOnce: some unusuality, so the FnBox is currently needed, and is unstable.

The 'move' keyboard must be used, which signals that all captures occurs by value.
This is required becaus any captures by reference would be dropped as soon as the
function exited, leaving invalid references in the closure.
*/

fn create_fn() -> Box<Fn()> {
    let text = "Fn".to_owned();

    Box::new(move || println!("This is a: {}", text))
}

fn create_fnmut() -> Box<FnMut()> {
    let text = "FnMut".to_owned();

    Box::new(move || println!("This is a: {}", text))
}

pub fn run() {
    //
    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();

    fn_plain();
    fn_mut();
}
