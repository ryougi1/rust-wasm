use std::fmt::{self, Display, Formatter};
/*
A tuple is a collection of values of different types.
Tuples are constructed using parentheses (),
and each tuple itself is a value with type signature (T1, T2, ...),
where T1, T2 are the types of its members.
Functions can use tuples to return multiple values, as tuples can hold any number of values.
*/
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // Can use 'let' to bind the members of a tuple to a variable
    let (integer, boolean) = pair;
    (boolean, integer)
}

// TODO: Activity
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "( {} {} )\n( {} {} )", self.0, self.1, self.2, self.3)
    }
}

fn transpose(matrix: Matrix) -> Matrix {
    Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
}

pub fn run() {
    // Tuple with a bunch of different types
    let long_tuple = (
        1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true,
    );

    // Values can be extracted from the tuple using tuple indexing
    println!("Long tuple first value: {}", long_tuple.0);
    println!("Second: {}", long_tuple.1);

    // Tuples can be members of tuples
    let tuple_of_tuples = ((1u8, 2u16, 3u32), (4u64, -1i8), -2i16);

    // Tuples are printable with debug format
    println!("Tuple of tuples: {:?}", tuple_of_tuples);

    // However, a tuple that's too long is not printable
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("too long tuple: {:?}", too_long_tuple);

    let pair = (1, true);
    println!("Pair is {:?}", pair);
    println!("Reverse pairs is: {:?}", reverse(pair));

    // Tuples with one single element require a comma to tell them apart from a literal
    println!("Just an integer: {:?}", (5u32));
    println!("One element tuple: {:?}", (5u32,));

    // Tuples can be destructured to create bindings
    let tuple = (1, "hello", 4.5, true);
    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("Matrix:\n{}", matrix);
    println!("Transpose:\n{}", transpose(matrix));
}
