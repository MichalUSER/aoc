use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::seat;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn get_seats() -> Vec<seat::Seat> {
    let mut seats_vec: Vec<seat::Seat> = Vec::new();
    if let Ok(lines) = read_lines("seats.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let s = seat::Seat {
                    identifier: ip.chars().collect()
                };
                seats_vec.push(s);
            }
        }
    }
    return seats_vec;
}
