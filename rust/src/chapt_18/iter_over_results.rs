/*
Iterating over Results

collect():
can take anything iterable, and turn it into a relevant collection.

filter_map<B, F>(self, f: F) -> FilterMap<Self, F>:
Creates an iterator that both filters and maps.
*/

pub fn run() {
    println!("\nIter over results\nFilter map");
    let strings = vec!["tofu", "93", "18"];
    let numbers: Vec<_> = strings
        .into_iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();
    println!("Results: {:?}", numbers);

    let strings = vec!["tofu", "93", "18"];
    let (numbers, errors): (Vec<_>, Vec<_>) = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .partition(Result::is_ok);
    println!("Partition:");
    println!("Numbers: {:?}", numbers);
    println!("Errors: {:?}", errors);
    let numbers: Vec<_> = numbers.into_iter().map(Result::unwrap).collect();
    let errors: Vec<_> = errors.into_iter().map(Result::unwrap_err).collect();
    println!("Unwrap partitions");
    println!("Numbers: {:?}", numbers);
    println!("Errors: {:?}", errors);
}
