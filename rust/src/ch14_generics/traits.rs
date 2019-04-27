/*
Traits can also be generic.

Here we define one which reimplements the Drop trait as a
generic method to drop itself and an input
*/

struct Empty;
struct Null;

// A trait generic over 'T'
trait DoubleDrop<T> {
    // Define a method on the caller type which takes an
    // additional single parameter 'T' and does nothing with it
    fn double_drop(self, _: T);
}

impl<T, U> DoubleDrop<T> for U {
    // Take ownership of both passed arguments,
    // deallocating both.
    fn double_drop(self, _: T) {}
}

pub fn run() {
    let empty = Empty;
    let null = Null;

    empty.double_drop(null);
}

//TODO: Revisit after learning about traits
