/*
Integers, floats, characters, strings, booleans, and the unit type () can be expressed
using literals.

Integers can, alternatively, be expressed using 0x, 0o, 0b.

Underscores can be inserted in numeric literals to improve readability, e.g. 1_000 is the same as 1000.
0.000_001 is the same as 0.000001.
*/
pub fn run() {
    println!("1 + 2 = {}", 1u32 + 2);

    println!("1 - 2 = {}", 1i32 - 2);

    // Short-circuiting boolean logic
    println!("true && false is {}", true && false);
    println!("true || false is {}", true || false);
    println!("!true is {}", !true);

    // Bitwise operations
    println!("0011 & 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 | 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // Use underscores to improve readability
    println!("One million is written as {}", 1_000_000u32);
}
