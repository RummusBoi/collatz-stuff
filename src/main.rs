const REMAINDER_TO_SHIFT_LEFT: [i64; 2] = [0, 1];
const REMAINDER_TO_SHIFT_RIGHT: [i64; 2] = [1, 0];
const REMAINDER_TO_MULTIPLIER: [i64; 2] = [0, 1];
const REMAINDER_TO_ADDITION: [i64; 2] = [0, 1];
const CACHE_SIZE: usize = 100000000;

static mut CACHED_CHAIN_LENGTHS: [i16; CACHE_SIZE] = [0; CACHE_SIZE];

fn compute_collatz_numbers(n: i64, buffer: &mut Vec<i64>) {
    let mut num = n;
    while num != 4 {
        let remainder = (num % 2) as usize;
        // println!("num: {}", num);
        // let res = num << REMAINDER_TO_SHIFT_AMOUNT[remainder];
        // println!("num shifted: {}", res);
        // println!("Remainder: {}", remainder);
        num = ((num << REMAINDER_TO_SHIFT_LEFT[remainder]) >> REMAINDER_TO_SHIFT_RIGHT[remainder])
            + num * REMAINDER_TO_MULTIPLIER[remainder]
            + REMAINDER_TO_ADDITION[remainder];
        buffer.push(num);
    }
}

fn find_highest_collatz_number(n: i64) -> i64 {
    let mut num = n;
    let mut max_num_found = 0;
    while num != 4 {
        let remainder = (num % 2) as usize;
        // println!("num: {}", num);
        // let res = num << REMAINDER_TO_SHIFT_AMOUNT[remainder];
        // println!("num shifted: {}", res);
        // println!("Remainder: {}", remainder);
        num = ((num << REMAINDER_TO_SHIFT_LEFT[remainder]) >> REMAINDER_TO_SHIFT_RIGHT[remainder])
            + num * REMAINDER_TO_MULTIPLIER[remainder]
            + REMAINDER_TO_ADDITION[remainder];
        max_num_found = num.max(max_num_found);
    }
    max_num_found
}

fn find_longest_collatz_number_chain(n: i64) -> i16 {
    let mut num = n;
    let mut chain_length = 0;
    let cast_cache_size = CACHE_SIZE as i64;
    while num != 4 {
        let remainder = (num % 2) as usize;
        // println!("num: {}", num);
        // let res = num << REMAINDER_TO_SHIFT_AMOUNT[remainder];
        // println!("num shifted: {}", res);
        // println!("Remainder: {}", remainder);
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
        CACHED_CHAIN_LENGTHS[n as usize] = chain_length;
    }
    return chain_length;
}

fn print_highest_number_chain() {
    let mut max_num_found = 0;
    let mut max_num_val = 0;
    for i in 1..10000 {
        let res = find_highest_collatz_number(i);
        if res > max_num_val {
            max_num_val = res;
            max_num_found = i;
        }
    }
    println!("{}", max_num_found);
    let mut buffer: Vec<i64> = Vec::with_capacity(100);
    compute_collatz_numbers(max_num_found, &mut buffer);
    println!("{:?}", buffer);
}

fn print_longest_chain() {
    let mut longest_chain_length = 0;
    let mut longest_chain_num = 0;
    for i in 1..CACHE_SIZE as i64 {
        let chain_length = find_longest_collatz_number_chain(i);
        if chain_length > longest_chain_length {
            longest_chain_length = chain_length;
            longest_chain_num = i;
        }
    }
    println!("Chain length: {}", longest_chain_length);
    println!("Value: {}", longest_chain_num);
    let mut buffer: Vec<i64> = Vec::with_capacity(100);
    compute_collatz_numbers(longest_chain_num, &mut buffer);
    println!("{:?}", buffer);
}

fn main() {
    print_longest_chain();
}
