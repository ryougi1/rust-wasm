/*
When working with generics, the type parameter often must use traits
as bounds to stipulate what functionality a type implements.
*/

// A trait which implements the print marker: '{:?}'
use std::fmt::{Debug, Display};

trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.length * self.height
    }
}

#[derive(Debug)]
struct Rectangle {
    length: f64,
    height: f64,
}

#[allow(dead_code)]
struct Triangle {
    length: f64,
    height: f64,
}

// The generic 'T' must implement 'Debug'. Regardless of the
// type, this will work properly
fn print_debug<T: Debug>(t: &T) {
    println!("{:?}", t);
}

// 'T' must implement 'HasArea'. Any function which meets the
// bound can access 'HasArea''s function 'area'
fn area<T: HasArea>(t: &T) -> f64 {
    t.area()
}

pub fn run() {
    let rectangle = Rectangle {
        length: 3.0,
        height: 4.0,
    };
    let _triangle = Triangle {
        length: 3.0,
        height: 4.0,
    };

    print_debug(&rectangle);
    println!("Area: {}", area(&rectangle));

    // Calling the two functions with triangle will not work
    // as Triangle does not implement 'Debug' or 'HasArea'.

    // !
    let string = "words";
    let array = [1, 2, 3];
    let vec = vec![1, 2, 3];

    compare_prints(&string);
    // compare_prints(&array); does not work because array does not implement Display
    compare_types(&array, &vec);
}

// ! Multiple bounds
/*
Multiple bounds can be applied with a +. Like normal, different
types are separated with ','.
*/

fn compare_prints<T: Debug + Display>(t: &T) {
    println!("Debug: `{:?}`", t);
    println!("Display: `{}`", t);
}

fn compare_types<T: Debug, U: Debug>(t: &T, u: &U) {
    println!("t: {:?}", t);
    println!("u: {:?}", u);
}
