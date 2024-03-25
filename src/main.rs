use std::{
    cmp::{max, max_by},
    env::args,
};

const REMAINDER_TO_SHIFT_LEFT: [i64; 2] = [0, 1];
const REMAINDER_TO_SHIFT_RIGHT: [i64; 2] = [1, 0];
const REMAINDER_TO_MULTIPLIER: [i64; 2] = [0, 1];
const REMAINDER_TO_ADDITION: [i64; 2] = [0, 1];
const CACHE_SIZE: usize = 1000000000;
const MAX_THREADS: u32 = 32;

static mut CACHED_CHAIN_LENGTHS: [i16; CACHE_SIZE] = [0; CACHE_SIZE];

fn compute_collatz_numbers(n: i64, buffer: &mut Vec<i64>) {
    let mut num = n;

    buffer.push(num);
    while num != 4 {
        let remainder = (num % 2) as usize;
        num = ((num << REMAINDER_TO_SHIFT_LEFT[remainder]) >> REMAINDER_TO_SHIFT_RIGHT[remainder])
            + num * REMAINDER_TO_MULTIPLIER[remainder]
            + REMAINDER_TO_ADDITION[remainder];
        buffer.push(num);
    }
}

fn find_longest_collatz_number_chain(start: i64, end: i64) -> (i64, i16) {
    let mut chain_buffer: Vec<i64> = Vec::with_capacity(1000);
    let mut longest_chain_index = 0;
    let mut longest_chain_val = 0;
    for index in start..end {
        // println!("Computing index {}", index);
        chain_buffer.clear();
        let result = find_longest_collatz_number_chain_inner(index, &mut chain_buffer);
        if result > longest_chain_val {
            longest_chain_index = index;
            longest_chain_val = result;
        }
    }
    return (longest_chain_index, longest_chain_val);
}

fn find_longest_collatz_number_chain_inner(n: i64, chain_buffer: &mut Vec<i64>) -> i16 {
    let mut num = n;
    let mut chain_length = 0;
    let cast_cache_size = CACHE_SIZE as i64;
    while num != 4 {
        chain_buffer.push(num);
        let remainder = (num % 2) as usize;

        num = ((num << REMAINDER_TO_SHIFT_LEFT[remainder]) >> REMAINDER_TO_SHIFT_RIGHT[remainder])
            + num * REMAINDER_TO_MULTIPLIER[remainder]
            + REMAINDER_TO_ADDITION[remainder];
        chain_length += 1;

        unsafe {
            if num < cast_cache_size {
                let cached_val = CACHED_CHAIN_LENGTHS[num as usize];
                if cached_val > 0 {
                    chain_length += cached_val;

                    break;
                }
            }
        }
    }

    unsafe {
        for (index, this_number) in chain_buffer.iter().rev().enumerate() {
            let this_chain_length = chain_length - (index as i16);
            if *this_number < CACHED_CHAIN_LENGTHS.len() as i64 {
                CACHED_CHAIN_LENGTHS[this_number.clone() as usize] = this_chain_length;
            }
        }
    }
    return chain_length;
}

fn print_longest_chain(iterations: u32) {
    let mut longest_chain_length = 0;
    let mut longest_chain_num = 0;
    // let mut chain_buffer: Vec<i64> = Vec::with_capacity(1000);

    if !(iterations % MAX_THREADS == 0) {
        panic!("Number of iterations must be multiple of {}", MAX_THREADS);
    }

    let iterations_per_thread = iterations / MAX_THREADS;

    let mut futures = vec![];

    for thread_id in 0..MAX_THREADS {
        futures.push(std::thread::spawn(move || {
            find_longest_collatz_number_chain(
                (iterations_per_thread * thread_id + 1).into(),
                (iterations_per_thread * (thread_id + 1) + 1).into(),
            )
        }))
    }

    let results: Vec<(i64, i16)> = futures.into_iter().map(|fut| fut.join().unwrap()).collect();
    let max_result = results.iter().max_by(|x, y| x.1.cmp(&y.1)).unwrap();
    let longest_chain_length = max_result.1;
    let longest_chain_num = max_result.0;
    // for i in 1..iterations as i64 {
    //     chain_buffer.clear();

    //     let chain_length = find_longest_collatz_number_chain(i, &mut chain_buffer);
    //     if chain_length > longest_chain_length {
    //         longest_chain_length = chain_length;
    //         longest_chain_num = i;
    //     }
    // }
    println!("Chain length: {}", longest_chain_length);
    println!("Value: {}", longest_chain_num);
    let mut buffer: Vec<i64> = Vec::with_capacity(100);
    compute_collatz_numbers(longest_chain_num, &mut buffer);
    // unsafe {
    //     // CACHED_CHAIN_LENGTHS.iter().for_each(|val| println!("{}", val));
    //     println!("{:?}", CACHED_CHAIN_LENGTHS);
    // }
    // println!("{:?}", buffer);
}

fn main() {
    let arguments: Vec<String> = args().collect();

    let iterations: u32 = arguments[1].parse().unwrap();

    let start = std::time::Instant::now();
    print_longest_chain(iterations);
    let end = std::time::Instant::now();
    let duration = (end - start).as_millis();
    println!("\nOperation took {}ms\n", duration);
}
