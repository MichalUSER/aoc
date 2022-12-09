/*
use std::collections::HashMap;
use std::time::Instant;

type Dict = HashMap<char, usize>;

fn to_priority<'a>(dict: &'a Dict, c: &'a char) -> &'a usize {
    dict.get(c).unwrap()
}

fn main() {
    let now = Instant::now();
    let dict: Dict = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(idx, c)| (c, idx + 1))
        .collect();
    let all_common = include_str!("../input.txt").lines().map(|l| {
        let chars: Vec<char> = l.chars().collect();
        let chars_priorities: Vec<usize> = chars.iter().map(|c| *to_priority(&dict, c)).collect();
        let len = chars.len();
        let first: Vec<usize> = (0..(len / 2))
            .map(|i| *chars_priorities.get(i).unwrap())
            .collect();
        let second: Vec<usize> = ((len / 2)..len)
            .map(|i| *chars_priorities.get(i).unwrap())
            .collect();
        let mut common = 0;
        for p in first {
            if second.contains(&p) {
                common = p;
                break;
            }
        }
        common
    });
    let sum: usize = all_common.sum();
    println!("sum: {}", sum);
    println!("elapsed: {}", now.elapsed().as_micros());
}*/
