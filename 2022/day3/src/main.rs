use std::time::Instant;
use std::collections::HashMap;

fn main() {
    let now = Instant::now();
    let dict: HashMap<char, usize> = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(idx, c)| (c, idx + 1))
        .collect();
    let priorities: Vec<Vec<usize>> = include_str!("../input.txt")
        .lines()
        .map(|l| -> Vec<usize> {
            l.chars()
                .into_iter()
                .map(|c| *dict.get(&c).unwrap())
                .collect()
        })
        .collect();
    let all_common = priorities
        .clone()
        .into_iter()
        .enumerate()
        .step_by(3)
        .map(|(idx, l)| {
            let first = priorities.get(idx + 1).unwrap();
            let second = priorities.get(idx + 2).unwrap();
            let mut common = 0;
            for p2 in l {
                if first.contains(&p2) && second.contains(&p2) {
                    common = p2;
                    break;
                }
            }
            common
        });
    let sum: usize = all_common.sum();
    println!("sum: {}", sum);
    println!("elapsed: {}", now.elapsed().as_micros());
}
