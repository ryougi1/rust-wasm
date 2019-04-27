/*
Variables in Rust own their own resources. Whenever an object goes out of scope,
its destructor is called and its owned resources are freed.

Because variables are in charge of freeing their own resources,
resources can only have one owner. Note that not all variables own resources,
e.g. references.

When doing assignments (let x = y) or passing function arguments
by value (foo(x)), the ownership of the resource is transferred. This is
known as a move.

After a resource has been moved, the previous owner can no longer
be used.
*/

// This function takes ownership of the heap allocated memory
fn destroy_box(my_box: Box<i32>) {
    println!("Destroying a box that contains {}", my_box);

    // `box` is destroyed and the memory freed
}

pub fn run() {
    // Stack allocated integer
    let x: u32 = 5;
    // Copy x into y. Does not move, because
    // integer does not implement the Drop trait OR
    // integer implements Copy trait not sure yet.
    let y = x;

    println!("x is {} and y is {}", x, y);

    // a is a pointer to a heap allocated integer
    let a = Box::new(5i32);
    println!("a contains {}", a);

    // Move a into b
    let b = a;
    // Both a and b are now pointers to the same heap allocated
    // date, but b is the new owner. a can no longer access the data.

    // Calling the function here will cause it to take ownership of b
    destroy_box(b);

    // Can no longer use b.

    // * Mutability can be changed when ownership is transferred
    let c = Box::new(3u32);
    let mut d = c;
    *d = 333;
    // As per usual, c is no longer usable since it's not the owner anymore
}
