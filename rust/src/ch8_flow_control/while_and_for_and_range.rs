#![allow(dead_code)]
// ! while
/*
Same as usual, skipped
*/

// ! for loops
/*
The for in construct can be used to iterate through an Iterator.
One way to create an iterator is to use the range notation a..b,
which yields values from a inclusive to b exclusive in steps of one.
*/

/*
The for in construct is able to interact with an Iterator in several ways.
The for loop will apply the into_iter function on the collection provided
to convert the collection into an iterator. The other functions available include
iter and iter_mut.

iter - borrows each element of the collection through each iteration.
Collection is untouched and available for reuse after the loop.

into_iter - consumes the collection so that on each iteration the exact data
is provided. Once consumed it is no longer available for reuse as it has been
'moved' within the loop.

iter_mut - mutably borrows each element of the collection,
allowing for the collection to be modified in place
*/

pub fn run() {
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    let mut names = vec!["JG", "JB", "SR", "MH"];
    for name in names.iter() {
        match name {
            &"JB" => println!("Ye boiii"),
            _ => println!("Welcome, {}", name),
        }
    }
    println!("Changed: {:?}", names);

    // for (e, name) in names.iter_mut().enumerate() {
    for name in names.iter_mut() {
        match name {
            &mut "MH" => *name = "NOMAIL",
            _ => println!("Welcome, {}", name),
        }
    }

    println!("Changed: {:?}", names);
}
