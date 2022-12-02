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

#[allow(dead_code)]
fn count_increases() -> i32 {
    let mut count = 0;
    let lines = read_lines("input.txt");
    let lines_vec: Vec<String> = lines.scan((), |_, l| l.ok()).collect();
    for (index, e) in lines_vec.iter().skip(1).enumerate() {
        let prev_num_string = &lines_vec[index as usize];
        let prev_num: i32 = prev_num_string.parse().unwrap();
        let e_num: i32 = e.parse().unwrap();
        if e_num > prev_num {
            count += 1;
        }
    }
    count
}

#[allow(dead_code)]
fn count_increases2() -> i32 {
    let mut increases = 0;
    let lines = read_lines("input.txt");
    let lines_vec: Vec<i32> = lines.map(|l| l.unwrap().parse::<i32>().unwrap()).collect();
    let mut sums: Vec<i32> = Vec::new();
    let mut sum: i32 = lines_vec.iter().take(3).sum();
    let mut index = 0;
    sums.push(sum);
    lines_vec.iter().skip(3).for_each(|num| {
        sum += num;
        sum -= lines_vec[index];
        index += 1;
        sums.push(sum);
    });
    for (index, e) in sums.iter().skip(1).enumerate() {
        let prev_num: i32 = sums[index];
        if e > &prev_num {
            increases += 1;
        }
    }
    println!("sums: {:?}", sums);
    increases
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        println!("depth increase: {}", count_increases());
    }

    #[test]
    fn test_2() {
        println!("depth increase: {}", count_increases2());
    }
}
