use std::time::{Instant};

mod path_loader;

//println!("Lenght: {}", path.get(0).unwrap().len());
//println!("First path: {:?}", path.get(0).unwrap());
fn main() {
    let now = Instant::now();
    let path = path_loader::get_path();
    let mut index = 1;
    let mut trees = 0;
    let mut first = true;
    let mut skip = true;
    for line in path {
        if !first {
            if !skip {
                let current = line.get(index).unwrap();
                if *current == 1 {
                    trees += 1;
                }
                index += 1;
                skip = true;
            } else {
                skip = false;
            }
        } else {
            first = false;
        }
    }
    println!("Amount of trees: {}", trees);
    println!("Elapsed: {}ms", now.elapsed().as_millis());
}
