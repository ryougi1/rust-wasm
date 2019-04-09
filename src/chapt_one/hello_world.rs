use std::fmt;

// ! Debug
// Derive the fmt::Debug implementation for 'Structure'
// 'Structure' is a structure which contains a single 'i32'
#[derive(Debug)]
struct Structure(i32);

// Put a 'Structure' inside 'Deep'. Make it also printable
#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

// ! Display
// A structure holding two numbers. `Debug` will be derived so the results can
// be contrasted with `Display`.
#[derive(Debug)]
struct MinMax(i64, i64);

// Implement `Display` for `MinMax`.
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "({}, {})", self.0, self.1)
    }
}

// Define a structure where the fields are nameable for comparison.
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

// Similarly, implement for Point2D
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

// TODO: Display Activity
#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

// ! Testcase: List
// TODO: List Activity
/*
Implementing fmt::Display for a structure where the elements must each be handled
sequentially is tricky. The problem is that each write! generates a
fmt::Result. Proper handling of this requires dealing with all the results.
Rust provides the ? operator for exactly this purpose.
*/

struct List(Vec<i32>); // Define structure 'List' containing a 'Vec'

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Extract the value using tuple indexing
        // and create a reference to `vec`.
        let vec = &self.0;

        write!(f, "[")?;

        // Iterate over `vec` in `v` while enumerating the iteration
        // count in `count`.
        for (count, v) in vec.iter().enumerate() {
            // For every element except the first, add a comma.
            // Use the ? operator, or try!, to return on errors.
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{count}: {v}", count = count, v = v)?;
        }

        // Close the opened bracket and return a fmt::Result value
        write!(f, "]")
    }
}

pub fn run() {
    println!("Hello, world!");

    let pi: f64 = 3.141592;
    println!("Pi is roughly {:.3}", pi);

    println!("Now {:?} will print!", Structure(3));
    println!("Now {:?} will print!", Deep(Structure(7)));

    let name = "Jimbo";
    let age = 23;
    let jimbo = Person { name, age };

    // Pretty print
    println!("{:#?}", jimbo);

    // ! Display
    let minmax = MinMax(0, 14);

    println!("Comparing structures: ");
    println!("Debug: {:?}", minmax);
    println!("Display: {}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);
    println!(
        "The big range is {b} and the small range is {s}",
        b = big_range,
        s = small_range
    );

    let point = Point2D { x: 3.3, y: 7.2 };
    println!("Compare points:");
    println!("Debug: {:?}", point);
    println!("Display: {}", point);

    let complex = Complex {
        real: 3.3,
        imag: 7.2,
    };

    println!("Compare complex:");
    println!("Debug: {:?}", complex);
    println!("Display: {}", complex);

    // ! Testcase: List

    let v = List(vec![1, 2, 3, 4, 5, 6, 7]);
    println!("List Display: {}", v);
}
