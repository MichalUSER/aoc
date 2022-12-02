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

fn convert(s: String) -> i32 {
    i32::from_str_radix(s.as_str(), 2).unwrap()
}

fn main() {
    let bin_nums: Vec<Vec<i8>> = read_lines("input.txt")
        .map(|n| {
            n.unwrap()
                .chars()
                .into_iter()
                .map(|c| match c {
                    '1' => 1,
                    _ => 0,
                })
                .collect()
        })
        .collect();
    let mut gamma_rate = String::new();
    let mut epsilon_rate = String::new();
    for i in 0..bin_nums[0].len() {
        //let mut one_count = bin_nums.iter().filter(|bits| bits[i] == 1).collect().count();
        let mut one_count = 0;
        let mut zero_count = 0;
        for bits in &bin_nums {
            match bits[i] {
                1 => one_count += 1,
                _ => zero_count += 1,
            }
        }
        if one_count > zero_count {
            gamma_rate.push('1');
            epsilon_rate.push('0')
        } else {
            gamma_rate.push('0');
            epsilon_rate.push('1')
        }
    }
    let num1 = convert(gamma_rate);
    let num2 = convert(epsilon_rate);
    println!("num1: {}", num1);
    println!("num2: {}", num2);
    println!("power consumption: {}", num1 * num2);
}
