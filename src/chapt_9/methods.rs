#![allow(dead_code)]

struct Point {
    x: f64,
    y: f64,
}

// Implementation block, all `Point` methods go in here
impl Point {
    // This is a static method
    // Static methods don't need to be called by an instance
    // These methods are generally used as constructors
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // Another static method, taking two arguments:
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // Instance method
    // &self is sugar for self: &Self, where Self is the type of the caller object.
    // In this case, 'Self' = Rectangle
    fn area(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // This method requires the caller object to be mutable
    // &mut self is sugar for self: &mut Self
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// Pair owns resource: two heap allocated integers
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // Method consumes the resources of the caller object
    fn destroy(self) {
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);
        // 'first' and 'second' are now out of scope and get freed
    }
}

pub fn run() {
    let rect = Rectangle {
        //Static methods are called using double colons
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // Call instance methods with the dot operator
    // The first argument &self is implicitly passed
    println!(
        "Rectangle perimiter: {} \nRectangle area: {}",
        rect.perimeter(),
        rect.area()
    );

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));
    pair.destroy();

    // Can't do pair.destroy(); since pair consumed in previous call
}
