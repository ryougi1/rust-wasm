/*
A bound can also be expressed using a where clause immediately
before the opening {, rather than at the type's first mention.
Additionally, where clauses can apply bounds to arbitrary types,
rather than just to type parameters.

Example case where a where clause is useful:
impl <A: TraitB + TraitC, D: TraitE + TraitF> MyTrait<A, D> for YourType {}

// Expressing bounds with a `where` clause
impl <A, D> MyTrait<A, D> for YourType where
    A: TraitB + TraitC,
    D: TraitE + TraitF {}
*/

use std::fmt::Debug;

trait PrintInOption {
    fn print_in_option(self);
}

impl<T> PrintInOption for T
where
    Option<T>: Debug,
{
    fn print_in_option(self) {
        println!("{:?}", Some(self));
    }
}

pub fn run() {
    let vec = vec![1, 2, 3];
    vec.print_in_option();
}
