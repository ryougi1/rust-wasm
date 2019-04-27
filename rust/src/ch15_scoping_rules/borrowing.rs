#[allow(dead_code)]
/*
Most of the time, we want to access data without taking ownership over it.
Can do so using the borrowing mechanism. Instead of passing objects by value T,
objects can be passed by reference &T.

References always point to valid objects, meaning while a reference to T exist,
T cannot be destroyed. Guaranteed via the brrow checker.
*/

fn eat_box(a_box: Box<i32>) {
    println!("Destroyed box {}", a_box);
}

fn borrow_box(a_box: &Box<i32>) {
    println!("Borrowed box {}", a_box);
}

pub fn run() {
    let b1 = Box::new(5i32);
    let b2 = Box::new(6i32);
    borrow_box(&b1);
    borrow_box(&b2);

    // Create inner scope
    {
        let my_ref = &b1;
        println!("my ref: {}", my_ref);
        // can't do eat_box(b1) since inner value is being borrowed
        // my_ref goes out of scope
    }

    // Since reference no longer frozen, can now eat_box
    eat_box(b1);

    // ! Mutable borrows
    let book = Book {
        author: "Douglas Hofstadter",
        title: "Gödel, Escher, Bach",
        year: 1979,
    };

    // * This creates a mutable copy because Book derives Clone, Copy
    let mut mutabook = book;
    borrow_book(&book); // Can immutably borrow immutable
    borrow_book(&mutabook); // Can immutably borrow mutable
    new_edition(&mut mutabook); // Can mutably borrow mutable

    // new_edition(&mut book) nono, how you gonna mutably borrow an immutable obj

    // * When data is immutably borrowed, it is frozen. Frozen data can't be modified
    // * via original object until all references go out of scope.
    // * Example with immutable variable above and mutable below

    let mut _my_int = 9i32;

    {
        let _borrowed = &_my_int;
        // Will compile for some reason, but should be frozen due to borrow
        //_my_int = 99;

        // _borrowed goes out of scope
    }
    _my_int = 99;
}

/*
Mutable data can be mutably borrowed using &mut T. These are mutable references
and gives read/write access to the borrower.

In contrast, &T, immutable reference, only gives read access.
 */

#[derive(Clone, Copy)]
struct Book {
    author: &'static str,
    title: &'static str,
    year: u32,
}

fn borrow_book(book: &Book) {
    println!(
        "I immutably borrowed {} - {} edition",
        book.title, book.year
    );
}

fn new_edition(book: &mut Book) {
    book.year = 2019;
    println!("I mutably borrowed {} - {} edition", book.title, book.year);
}
