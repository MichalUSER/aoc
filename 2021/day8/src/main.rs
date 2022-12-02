use std::collections::HashMap;

fn main() {
    let after_delimiter: Vec<&str> = include_str!("../example.txt")
        .trim()
        .lines()
        .map(|l| l.split_once(" | ").unwrap().1)
        .collect();
    for s in &after_delimiter {
        println!("{}", s);
    }
    println!();
    // Find unique text
    let mut text_count: HashMap<&str, i32> = HashMap::new();
    after_delimiter.iter().for_each(|nums| {
        let split: Vec<&str> = nums.split(' ').collect();
        for s in split {
            *text_count.entry(s).or_insert(0) += 1;
        }
    });
    println!("{:?}", text_count);
}
