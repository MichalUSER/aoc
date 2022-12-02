mod numbers_loader;

use std::time::Instant;

fn has_sum(n: i64, preamble: &Vec<i64>) -> bool {
    for e in preamble.iter() {
        if preamble.contains(&(n - e)) {
            return true;
        }
    }
    return false;
}

fn main() {
    let now = Instant::now();
    let numbers = numbers_loader::get_numbers();
    let numbers_len = numbers.len();
    let preamble_length = 25;
    let mut preamble_array: Vec<i64> = Vec::new();
    for i in 0..numbers_len {
        let n = numbers[i];
        if preamble_array.len() < preamble_length {
            preamble_array.push(n);
        } else {
            if !has_sum(n, &preamble_array) {
                println!("Number: {}", n);
            }
            preamble_array.drain(0..1);
            preamble_array.push(n);
        }
    }
    println!("Elapsed: {}ms", now.elapsed().as_millis());
}
