#![allow(dead_code)]

use List::*;

enum List {
    //Cons: Tuple struct that wraps an element to a pointer to the next node
    Cons(u32, Box<List>), // Box: allocates memory on the heap and then places x into it.
    //Nil: A nde that signifies the end of the linked list
    Nil,
}

// Methods can be attached to an enum
impl List {
    // Create an empty list
    fn new() -> List {
        Nil
    }

    //Consume a list, and return the same list with a new element at its front
    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    // Return the length of the list
    fn len(&self) -> u32 {
        // `self` has to be matched, because the behavior of this method
        // depends on the variant of `self`
        // `self` has type `&List`, and `*self` has type `List`, matching on a
        // concrete type `T` is preferred over a match on a reference `&T`
        match *self {
            // Can't take ownership of the tail, because `self` is borrowed;
            // instead take a reference to the tail
            Cons(_, ref tail) => 1 + tail.len(),
            // Base case: empty list has length 0
            Nil => 0,
        }
    }

    // Return representation of the list as a heap-allocated string
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => format!("{}, {}", head, tail.stringify()),
            Nil => format!("Nil"),
        }
    }
}

pub fn run() {
    let mut list: List = List::new();

    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("Linked list has length: {}", list.len());
    println!("Linked list: {}", list.stringify());
}
