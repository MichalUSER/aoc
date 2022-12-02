// part2

use std::time::{Instant};

mod numbers;

fn find_sum_nums(wanted_num: i32) -> Result<(i32, i32), String> {
    let mut checked_numbers: Vec<i32> = Vec::new();
    for num in numbers::get_numbers().into_iter() {
        if checked_numbers.contains(&(wanted_num - num)) {
            return Ok((num, wanted_num - num))
        }
        checked_numbers.push(num);
    }
    Err(String::from("Wanted number not found"))
}

fn main() {
    let now = Instant::now();
    for num in numbers::get_numbers().into_iter() {
        //if checked_numbers.contains(&(2020 - num)) {
            //let num_to_find = (2020 - num) * num;
        //}
        let num_to_find = 2020 - num;
        let result = find_sum_nums(num_to_find);
        match result {
            Ok(two_nums) => println!("Result: {}", num * two_nums.0 * two_nums.1),
            Err(_) => (),
        }
    }
    println!("Elapsed: {}", now.elapsed().as_millis());
}
