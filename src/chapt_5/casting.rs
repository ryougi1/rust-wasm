/*
Rust provides no implicit type conversion (coercion) between primitive types.
But, explicit type conversion (casting) can be performed using the as keyword.

Rules for converting between integral types follow C conventions generally,
except in cases where C has undefined behavior.
The behavior of all casts between integral types is well defined in Rust.
*/

// Suppress all warnings from casts which overflow.
#![allow(overflowing_literals)]

pub fn run() {
    let decimal = 65.4321_f32;

    // No implicit conversion!
    // let integer: u8 = decimal;

    // Explicit
    let integer = decimal as u8;
    let character = integer as char;

    println!("Casting: {} --> {} --> {}", decimal, integer, character);

    // 1000 (default i32) already fits in a u16
    println!("1000 as a u16 is: {}", 1000 as u16);

    // Under the hood, the first 8 least significant bits are kept,
    // while the rest towards the most significat bit get truncated
    println!("1000 in binary: {:16b}", 1000);
    println!("1000 as a u8 is: {}", 1000 as u8);
    println!("1000 as u8 in binary: {:8b}", 1000 as u8);

    // -1 + 256 = 255
    // When casting any value to an unsigned type T,
    // std::T::MAX + 1 is added or subtracted
    // until the value fits into the new type
    println!("-1 as a u8 is: {}", (-1i8) as u8);

    // For positive numbers, this is the same as the modulus
    println!("1000 mod 256 is : {}", 1000 % 256);

    // When casting to a signed type, the (bitwise) result is the same as
    // first casting to the corresponding unsigned type. If the most significant
    // bit of that value is 1, then the value is negative.

    // Unless it already fits, of course.
    println!(" 128 as a i16 is: {}", 128 as i16);
    // 128 as u8 -> 128, whose two's complement in eight bits is:
    println!(" 128 as a i8 is : {}", 128 as i8);

    // repeating the example above
    // 1000 as u8 -> 232
    println!("1000 as a u8 is : {}", 1000 as u8);
    // and the two's complement of 232 is -24
    println!(" 232 as a i8 is : {}", 232 as i8);
}
