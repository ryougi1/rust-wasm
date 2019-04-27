/*
A lifetime is a construct the compiler uses to ensure all borrows are valid.
Specifically, a variable's lifetime begins when it is created and ends when
it is destroyed.

While lifetimes and scopes are often referred to together, they are not the same.

For example, we borrow a variable via &. The borrow has a lifetime that is
determined by where it is declared. As a result, the borrow is valid as long
as it ends before the lender is destroyed. However, the scope of the borrow
is determined by where the reference is used.

The syntax for explicitly annotating a lifetime uses an apostrophe character
as follows:
'foo' has a lifetime parameter 'a
foo<'a>

Similar to closures, using lifetimes requires generics. Additionally, this
lifetime syntax indicates that the lifetime of foo may not exceed that of 'a.

Explicit annotation of a type has the form:
&'a T
where 'a has already been introduced.

In cases with multiple lifetimes, the syntax is similar:
foo<'a, 'b>
In this case, lifetime of foo cannot exceed that of either 'a or 'b
*/

pub fn run() {
    // * Functions
    println!("\n\n");
    let x = 7;
    let y = 9;

    print_one(&x);
    print_multi(&x, &y);

    let z = pass_x(&x, &y);
    print_one(&z);

    let mut t = 3;
    add_one(&mut t);
    print_one(&t);

    // * Methods
    let mut owner = Owner(18);
    owner.add_one();
    owner.print();

    // * Structs
    let x = 18;
    let y = 15;

    let single = Borrowed(&x);
    let double = NamedBorrowed { x: &x, y: &y };
    let reference = Either::Ref(&x);
    let number = Either::Num(y);

    println!("x is borrowed in {:?}", single);
    println!("x and y are borrowed in {:?}", double);
    println!("x is borrowed in {:?}", reference);
    println!("y is *not* borrowed in {:?}", number);
}

// ! Functions
/*
Function signatures with lifetimes have a few constraints:
1. Any reference must have an annotated lifetime
2. Any reference being returned must have the same lifetime as an input or be static
*/

// One input reference with lifetime `'a` which must live
// at least as long as the function.
fn print_one<'a>(x: &'a i32) {
    println!("`print_one`: x is {}", x);
}

// Mutable references are possible with lifetimes as well.
fn add_one<'a>(x: &'a mut i32) {
    *x += 1;
}

// Multiple elements with different lifetimes. In this case, it
// would be fine for both to have the same lifetime `'a`, but
// in more complex cases, different lifetimes may be required.
fn print_multi<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("`print_multi`: x is {}, y is {}", x, y);
}

// Returning references that have been passed in is acceptable.
// However, the correct lifetime must be returned.
fn pass_x<'a, 'b>(x: &'a i32, _: &'b i32) -> &'a i32 {
    x
}

// ? fn invalid_output<'a>() -> &'a String { &String::from("foo") }
// The above is invalid: `'a` must live longer than the function.
// Here, `&String::from("foo")` would create a `String`, followed by a
// reference. Then the data is dropped upon exiting the scope, leaving
// a reference to invalid data to be returned.

// ! Methods
// Methods are annotated similarly to functions
struct Owner(i32);

impl Owner {
    fn add_one<'a>(&'a mut self) {
        self.0 += 1;
    }

    fn print<'a>(&'a self) {
        println!("print: {}", self.0);
    }
}

// ! Structs
// Annotation of lifetimes in structures are also similar to functions

// A type `Borrowed` which houses a reference to an
// `i32`. The reference to `i32` must outlive `Borrowed`.
#[derive(Debug)]
struct Borrowed<'a>(&'a i32);

// Similarly, both references here must outlive this structure.
#[derive(Debug)]
struct NamedBorrowed<'a> {
    x: &'a i32,
    y: &'a i32,
}

// An enum which is either an `i32` or a reference to one.
#[derive(Debug)]
enum Either<'a> {
    Num(i32),
    Ref(&'a i32),
}
