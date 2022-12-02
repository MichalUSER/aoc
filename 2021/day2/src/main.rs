use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn read_lines<P>(filename: P) -> io::Lines<io::BufReader<File>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("can't read file");
    io::BufReader::new(file).lines()
}

// Part 1
/*fn main() {
    let mut x = 0;
    let mut depth = 0;
    read_lines("input.txt").for_each(|l| {
        let split = l.unwrap();
        let split: Vec<&str> = split.split(' ').collect();
        let movement: i32 = split[1].parse().unwrap();
        match split[0] {
            "forward" => x += movement,
            "up" => depth -= movement,
            "down" => depth += movement,
            _ => ()
        }
    });
    println!("multiply pos by depth: {}", x * depth)
}*/

// Part 2
fn main() {
    let mut x = 0;
    let mut depth = 0;
    let mut aim = 0;
    read_lines("input.txt").for_each(|l| {
        let split = l.unwrap();
        let split: Vec<&str> = split.split(' ').collect();
        let movement: i32 = split[1].parse().unwrap();
        match split[0] {
            "forward" => {
                x += movement;
                depth += aim * movement;
            },
            "up" => aim -= movement,
            "down" => aim += movement,
            _ => ()
        }
    });
    println!("pos: {}, depth: {}", x, depth);
    println!("multiply pos by depth: {}", x * depth)
}
