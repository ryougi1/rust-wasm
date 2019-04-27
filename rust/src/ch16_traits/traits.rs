/*
A trait is a collection of methods defined for an unknown type: Self.
They can access other methods declared in the same trait.

Traits can be implemented for any data type. In the example below,
we define Animal, a group of methods. The Animal trait is then implemented
for the Sheep data type, allowing the use of methods from Animal with a Sheep.
*/

struct Sheep {
    naked: bool,
    name: &'static str,
}

trait Animal {
    // Static method signature; `Self` refers to the implementor type.
    fn new(name: &'static str) -> Self;

    // Instance method signatures; these will return a string.
    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    // Traits can provide default method definitions.
    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}

impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }

    fn shear(&mut self) {
        if self.is_naked() {
            // Implementor methods can use the implementor's trait methods.
            println!("{} is already naked...", self.name());
        } else {
            println!("{} gets a haircut!", self.name);

            self.naked = true;
        }
    }
}

// Implement the `Animal` trait for `Sheep`.
impl Animal for Sheep {
    // `Self` is the implementor type: `Sheep`.
    fn new(name: &'static str) -> Sheep {
        Sheep {
            name: name,
            naked: false,
        }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        if self.is_naked() {
            "baaaaah?"
        } else {
            "baaaaah!"
        }
    }

    // Default trait methods can be overridden.
    fn talk(&self) {
        // For example, we can add some quiet contemplation.
        println!("{} pauses briefly... {}", self.name, self.noise());
    }
}

pub fn run() {
    // Type annotation is necessary in this case.
    let mut dolly: Sheep = Animal::new("Dolly");

    dolly.talk();
    dolly.shear();
    dolly.talk();
}

// ! Derive
/*
The compiler is capable of provoding basic implementations for some traits
via the #[derive] attribute. These traits can still be manually implemented
if a more complex behaviour is required.

List of derivable traits:
Comparison traits: Eq, PartialEq, Ord, PartialOrd
Clone, to create T from &T via a copy.
Copy, to give a type 'copy semantics' instead of 'move semantics'
Hash, to compute a hash from &T.
Default, to create an empty instance of a data type.
Debug, to format a value using the {:?} formatter.
*/

// ! Operator Overloading
/*
In Rust, many of the operators can be overloaded via traits.
That is, some operators can be used to accomplish different tasks
based on their input arguments. This is possible because operators are
syntactic sugar for method calls. For example, the + operator in a + b
calls the add method (as in a.add(b)). This add method is part of the Add trait.
Hence, the + operator can be used by any implementor of the Add trait.

A list of the traits, such as Add, that overload operators can be found in core::ops.
*/

// ! Drop
/*
The Drop trait only has one method: drop, which is called automatically
when an object goes out of scope. The main use of the Drop trait is to
free the resources that the implementor instance owns.

Box, Vec, String, File, and Process are some examples of types that implement
the Drop trait to free resources. The Drop trait can also be manually implemented
for any custom data type.
*/
