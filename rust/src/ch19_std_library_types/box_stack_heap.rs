/*
As previously mentioned, values can be boxed (allocated in the heap) by 
creating a Box<T>. When a box goes out of scope, its destructor is called, 
the inner object is destroyed, and the memory in the heap is freed.

Boxed values can be dereferenced using the *operator; 
this removes one layer of indirection.
*/

use std::mem;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

#[allow(dead_code)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

fn origin() -> Point {
    Point {x: 0.0, y: 0.0}
}

fn boxed_origin() -> Box<Point> {
    Box::new(Point {x: 0.0, y: 0.0})
}

pub fn run() {
    // All stack allocated
    let point: Point = origin();
    let rectangle: Rectangle = Rectangle {
        p1: origin(),
        p2: Point {x: 3.0, y: 4.0}
    };

    // Heap allocated rect
    let boxed_rect: Box<Rectangle> = Box::new(Rectangle {
        p1: origin(),
        p2: origin()
    });

    // Output of functions can be boxed
    let boxed_point: Box<Point> = Box::new(origin());

    // Double indirection
    let boxbox_point: Box<Box<Point>> = Box::new(boxed_origin());

    println!("Point occupies {} bytes in the stack",
             mem::size_of_val(&point));
    println!("Rectangle occupies {} bytes in the stack",
             mem::size_of_val(&rectangle));

    // box size = pointer size
    println!("Boxed point occupies {} bytes in the stack",
             mem::size_of_val(&boxed_point));
    println!("Boxed rectangle occupies {} bytes in the stack",
             mem::size_of_val(&boxed_rect));
    println!("Boxed box occupies {} bytes in the stack",
             mem::size_of_val(&boxbox_point));
    
    // Copy the data contained in `boxed_point` into `unboxed_point`
    let unboxed_point: Point = *boxed_point;
    println!("Unboxed point occupies {} bytes in the stack",
             mem::size_of_val(&unboxed_point));
}