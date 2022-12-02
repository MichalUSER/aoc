mod answers_loader;

use std::time::Instant;

fn main() {
    let now = Instant::now();
    println!("Count: {}", answers_loader::get_answers());
    println!("Elapsed: {}ms", now.elapsed().as_millis());
}
