// ! if else
/*
Same as usual, but condition doesn't need to be surrounded by parentheses,
and each condition is followed by a block.
*/

// ! loop
/*
Rust provides a look keyword to indicate an infinite loop.
The break statement can be used to exit a loop at any time,
whereas the continue statement can be used to skip the remainder
of the iteration and start a new one.
*/

/*
It is possible to break or continue outer loops when dealing with
nested loops. Loops must be annotated with some 'label, and the label
must be passed to the break/continue statement
*/

/*
Loops can return, just it after the break.
*/

pub fn run() {
    let n = 5;

    let big_n = if n < 10 && n > -10 { 10 * n } else { n / 2 };
    println!("{} --> {}", n, big_n);

    let mut count = 0u32;
    loop {
        count += 1;
        if count == 3 {
            println!("three");
            continue;
        }

        println!("{}", count);

        if count == 6 {
            println!("No more");
            break;
        }
    }

    'outer: loop {
        println!("Entered outer loop");

        'inner: loop {
            println!("Entered inner loop");
            break 'outer;
        }
        println!("Will never get here");
    }
    println!("Exited outer loop");

    let mut count = 0;
    let result = loop {
        count += 1;
        if count == 10 {
            break count * 2;
        }
    };

    assert_eq!(result, 20);
}
