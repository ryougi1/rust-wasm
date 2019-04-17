#![allow(dead_code)]
/*
The From and Into traits are inherently linked,
and this is actually part of its implementation.
If you are able to convert type A from type B,
then it should be easy to believe that we should be able to convert type B to type A.
*/

use std::convert::From;
use std::string::ToString;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

// ! ToString
// To convert any type to a String it is as simple as
// implementing the ToString trait for the type
struct Circle {
    radius: i32,
}

impl ToString for Circle {
    fn to_string(&self) -> String {
        format!("Circle of radius {:?}", self.radius)
    }
}

// ! Parsing a String
/*
The idiomatic approach to converting a string into a number is to
use the parse function and provide the type for the function to parse
the string value into. Works so long as the type specified implements the FromStr trait.
This is already implemented for numberous types within the standard library.
*/
pub fn run() {
    let my_str = "hello";
    let my_string = String::from(my_str);
    println!("{} vs. {}", my_str, my_string);

    let num = Number::from(30);
    println!("My number is {:?}", num);

    /*
    The Into trait is simply the reciprocal of the From trait.
    That is, if you have implemented the From trait for your type
    you get the Into implementation for free.
    */
    let int = 5i32;
    let num: Number = int.into();
    println!("My number is {:?}", num);

    let circle: Circle = Circle { radius: 6 };
    println!("{}", circle.to_string());

    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();
    let sum = parsed + turbo_parsed;
    println!("Sum: {}", sum);
}
