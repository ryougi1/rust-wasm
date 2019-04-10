/*
An array is a collection of objects of the same type T, stored in contiguous memory.
Arrays are created using brackets [], and their size, which is known at compile time,
is part of their type signature [T; size].

Slices are similar to arrays, but their size is not known at compile time.
Instead, a slice is a two-word object, the first word is a pointer to the data,
and the second word is the length of the slice. The word size is the same as usize,
determined by the processor architecture eg 64 bits on an x86-64.
Slices can be used to borrow a section of an array, and have the type signature &[T].
*/

#![allow(dead_code)]

use std::mem;

//This function borrows a slice
fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

pub fn run() {
    //Fixed size array
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // All elements can be initialized to the same value
    let ys: [i32; 15] = [8; 15];
    println!("xs: {:?}", xs);
    println!("ys: {:?}", ys);

    // 'len' returns the size of the array
    println!("xs len: {}", xs.len());

    // Arrays are stack allocated
    println!("xs occupies {} bytes", mem::size_of_val(&xs));

    // Borrowing for slice
    analyze_slice(&xs);
    analyze_slice(&xs[0..3]);
}
