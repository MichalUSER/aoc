/*
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let count = include_str!("../input.txt").lines().fold(0, |acc, l| {
        let (first, second) = l.split_once(',').unwrap();
        let (s1, s2) = first.split_once('-').unwrap();
        let (s3, s4) = second.split_once('-').unwrap();
        let s1: u32 = s1.parse().unwrap();
        let s2: u32 = s2.parse().unwrap();
        let s3: u32 = s3.parse().unwrap();
        let s4: u32 = s4.parse().unwrap();
        if (s1 >= s3 && s2 <= s4) || (s3 >= s1 && s4 <= s2) {
            acc + 1
        } else {
            acc
        }
    });
    println!("count: {}", count);
    println!("elapsed: {}", now.elapsed().as_micros());
}*/
