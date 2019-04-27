/*

Generics is the topic of generalizing types and functionalities to
broader cases. This is extremely useful for reducing code duplication in
many ways, but can call for rather involving syntax. Namely, being generic
requires taking great care to specify over which types a generic type is
actually considered valid. The simplest and most common use of generics is
for type parameters.

A type parameter is specified as generic by the use of angle brackets and
upper camel case: <Aaa, Bbb, ...>. "Generic type parameters" are typically
represented as <T>. In Rust, "generic" also describes anything that accepts
one or more generic type parameters <T>. Any type specified as a generic
type parameter is generic, and everything else is concrete (non-generic).
*/

// A concrete type 'A'
struct A;

// In defining the type 'Single', the first use of 'A' is not preceded by
// <A>, there for 'Single' is a concrete type, and 'A' is defined as above
struct Single(A);

// Here, <T> precedes the first use of 'T', so GenericSingle is a
// generic type. Because 'T' is generic, it could be anythig,
// including the concrete type 'A' defined at the top
struct GenericSingle<T>(T);

pub fn run() {
    // Single is concrete and explicitly takes 'A'
    let _single = Single(A);

    // Here, GenericSingle has a type parameter explicitly specified
    let _char: GenericSingle<char> = GenericSingle('a');
    // Can also have type parameter implicitly specified
    let _t = GenericSingle(A); // Uses A defined at the top
    let _i32 = GenericSingle(6); // Uses `i32`.
    let _char = GenericSingle('a'); // Uses `char`.
}
