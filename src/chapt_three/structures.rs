#![allow(dead_code)]

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

// A unit struct
struct Nil;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
struct Rectangle {
    p1: Point,
    p2: Point,
}

pub fn run() {
    // Create struct with field init shorthand
    let name = "Jimbo";
    let age = 23;
    let jimbo = Person { name, age };

    println!("{:?}", jimbo);

    // Instantiate a 'Point'
    let point: Point = Point { x: 0.3, y: 0.4 };

    // Access the fields of the 'Point' struct instance
    println!("First point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our other one
    let new_point: Point = Point { x: 0.1, ..point };
    // ! Can't say Point { ..point, y: 0.1} because can't use comma after base struct
    println!(
        "Second point coordinates: ({}, {})",
        new_point.x, new_point.y
    );

    // Destructure the point
    let Point { x: my_x, y: my_y } = point;
    println!("my_x: {}, my_y: {}", my_x, my_y);

    // Instantiate a unit struct
    let _nil: Nil = Nil;

    println!("Area: {}", rect_area(&square(point, 0.3f32)));
}
// ! Activity
fn square(p: Point, side: f32) -> Rectangle {
    let new_p: Point = Point {
        x: p.x + side,
        y: p.y + side,
    };

    let rect: Rectangle = Rectangle { p1: p, p2: new_p };
    rect
}

fn rect_area(rect: &Rectangle) -> f32 {
    // Destruct
    let Rectangle {
        p1: point_1,
        p2: point_2,
    } = rect;

    println!(
        "Received rect with p1: ({}, {}) and p2: ({}, {})",
        point_1.x, point_1.y, point_2.x, point_2.y
    );

    // let area: f32 = ((rect.p1.x - rect.p2.x) * (rect.p1.y - rect.p2.y)).abs();
    let area: f32 = ((point_1.x - point_2.x) * (point_1.y - point_2.y)).abs();
    area
}
