/*
Type Option represents an optional value: either Some which contains a value,
or None which does not.

For some use cases, when matching enums, match is awkward.
if let is cleaner and allows for various failure options to be specified
*/
pub fn run() {
    // All have type 'Option<i32>
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    // If 'number' destructure into 'Some(i),
    // then evaluate the block ('{}')
    if let Some(i) = number {
        println!("Macthed {:?}", i);
    }

    if let Some(i) = letter {
        println!("Matched {:?}", i);
    } else {
        println!("Didn't match");
    }

    // In the same way, if let can be used to match any enum value
}
