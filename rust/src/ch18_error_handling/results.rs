/*
Result is a richer version of the Option type that describes possible error
instead of possible absence.

That is, Result<T, E> could have one of two outcomes:

Ok<T>: An element T was found
Err<E>: An error was found with element E

Like Option, Result has many methods associated with it.
unwrap(), for example, either yields the element T or panics.
For case handling, there are many combinators between Result and Option that overlap.

In working with Rust, you will likely encounter methods that return the Result type,
such as the parse() method. It might not always be possible to parse a string into
the other type, so parse() returns a Result indicating possible failure.
*/

// ! Using Result in main
/*
However main is also able to have a return type of Result.
If an error occurs within the main function it will return an error code and print a
debug representation of the error (using the Debug trait).
*/

use std::num::ParseIntError;

// With the return type rewritten, we use pattern matching without `unwrap()`.
fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    match first_number_str.parse::<i32>() {
        Ok(first_number) => match second_number_str.parse::<i32>() {
            Ok(second_number) => Ok(first_number * second_number),
            Err(e) => Err(e),
        },
        Err(e) => Err(e),
    }
}

// map and and_then exist for Result too
fn multiply_2(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    first_number_str.parse::<i32>().and_then(|first_number| {
        second_number_str
            .parse::<i32>()
            .map(|second_number| first_number * second_number)
    })
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

pub fn run() {
    // This still presents a reasonable answer.
    let twenty = multiply("10", "2");
    print(twenty);

    // The following now provides a much more helpful error message.
    let tt = multiply("t", "2");
    print(tt);

    let forty = multiply_2("40", "1");
    print(forty);

    print_alias(multiply_alias("2", "4"));
}

// ! aliases for Result
/*
How about when we want to reuse a specific Result type many times?
Recall that Rust allows us to create aliases. Conveniently, we can
define one for the specific Result in question.
*/

// Define a generic alias for a `Result` with the error type `ParseIntError`.
type AliasedResult<T> = Result<T, ParseIntError>;

// Use the above alias to refer to our specific `Result` type.
fn multiply_alias(first_number_str: &str, second_number_str: &str) -> AliasedResult<i32> {
    first_number_str.parse::<i32>().and_then(|first_number| {
        second_number_str
            .parse::<i32>()
            .map(|second_number| first_number * second_number)
    })
}

// Here, the alias again allows us to save some space.
fn print_alias(result: AliasedResult<i32>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}
