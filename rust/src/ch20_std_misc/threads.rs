use std::thread;

static NTHREADS: i32 = 10;
static NBRS_PER_CHUNK: usize = 35;

// This is the `main` thread
pub fn run() {
    // Make a vector to hold the children which are spawned.
    let mut children = vec![];

    for i in 0..NTHREADS {
        // Spin up another thread
        children.push(thread::spawn(move || {
            println!("this is thread number {}", i);
        }));
    }

    for child in children {
        // Wait for the thread to finish. Returns a result.
        let _ = child.join();
    }

    // ! Bigger example

    // Make a vector to hold the child-threads which we will spawn.
    let mut children = vec![];

    // We will calculate the sum of all digits via a threaded map-reduce algorithm.
    // Each whitespace separated chunk will be handled in a different thread.
    let mut data = "8696789773741647185329732705036495911861322575564723963297542624962850708562347018608519079606900147256393839796670710609417278323874766921952380795257888236525459303330302837584953271357440410488978857342978126992021643898087354880841372095653216278424637452589860345374828574668";
    let mut count = 0;
    while data.len() > NBRS_PER_CHUNK {
        let (first, second) = data.split_at(NBRS_PER_CHUNK);
        println!("data seg: {} is {}", count, first);
        // Pref move this out to a function
        children.push(thread::spawn(move || -> u32 {
            let result: u32 = first
                .chars()
                .map(|c| c.to_digit(10).expect("should be digit"))
                .sum();
            println!("processed seg {}: result={}", count, result);
            result
        }));
        // So I don't have to repeat myself as such:
        if second.len() <= NBRS_PER_CHUNK {
            children.push(thread::spawn(move || -> u32 {
                let result: u32 = second
                    .chars()
                    .map(|c| c.to_digit(10).expect("should be digit"))
                    .sum();
                println!("processed seg {}: result={}", count, result);
                result
            }));
        }
        data = second;
        count += 1;
    }

    // for (i, data_segment) in data.split_whitespace().enumerate() {
    //     println!("data segment {} is \"{}\"", i, data_segment);

    //     // spawn() returns a handle to the new thread,
    //     // which we MUST keep to access the returned value.
    //     // 'move || -> u32':
    //     // * takes no arguments ('||')
    //     // * takes ownership of its captured variables ('move')
    //     // * returns an unsigned 32-bit integer ('-> u32')
    //     children.push(thread::spawn(move || -> u32 {
    //         // Calculate the intermediate sum of this segment:
    //         let result = data_segment
    //             .chars()
    //             .map(|c| c.to_digit(10).expect("should be a digit"))
    //             .sum();
    //         // println! locks stdout, so no text-interleaving occurs
    //         println!("processed segment {}, result={}", i, result);
    //         result
    //     }));
    // }

    // collect each thread's intermediate results into a new Vec
    let mut intermediate_sums = vec![];
    for child in children {
        // collect each child thread's return-value
        let intermediate_sum = child.join().unwrap();
        intermediate_sums.push(intermediate_sum);
    }

    // combine all intermediate sums into a single final sum.
    let final_result: u32 = intermediate_sums.iter().sum();

    println!("Final sum result: {}", final_result);
}

/*
Assignments
It is not wise to let our number of threads depend on user inputted data.
What if the user decides to insert a lot of spaces? Do we really want to spawn 2,000 threads?
Modify the program so that the data is always chunked into a limited number of chunks,
defined by a static constant at the beginning of the program.
*/
