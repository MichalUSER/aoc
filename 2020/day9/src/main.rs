mod numbers_loader;

use std::time::Instant;

// My original solution
//fn find_numbers(target_number: i64, numbers: &Vec<i64>) -> Option<Vec<i64>> {
    //let mut sums: Vec<i64> = Vec::new();
    //let mut sum = 0;
    //for e in numbers.iter() {
        //if sum < target_number {
            //sums.push(*e);
            //sum += *e;
        //} else {
            //sum -= sums[0];
            //sums.drain(0..1);
            //if sum == target_number {
                //return Some(sums);
            //}
            //sums.push(*e);
            //sum += *e;
        //}
    //}
    //return None;
//}


fn find_numbers(target_number: i64, numbers: &Vec<i64>) -> Option<&[i64]> {
    let mut start = 0;
    let mut end = 1;
    let mut sum: i64 = numbers[start..=end].iter().sum();

    while end < numbers.len() {
        if sum == target_number {
            return Some(&numbers[start..=end]);
        }
        if sum > target_number {
            sum -= numbers[start];
            start += 1;
        } else {
            end += 1;
            sum += numbers[end];
        }
    }
    None
}

fn get_min_and_max(sums: &[i64]) -> (i64, i64) {
    let mut min = sums[0];
    let mut max = sums[0];
    for e in sums.iter() {
        if *e < min {
            min = *e;
        } else if *e > max {
            max = *e;
        }
    }
    return (min, max);
}

fn main() {
    let now = Instant::now();
    let numbers = numbers_loader::get_numbers();
    let invalid_number = 26134589;
    let sums_option = find_numbers(invalid_number, &numbers);
    //println!("Sums: {:?}", sums_option);
    if let Some(sums) = sums_option {
        let min_and_max = get_min_and_max(sums);
        println!("Encryption weakness: {}", min_and_max.0 + min_and_max.1);
    } else {
        println!("Not found");
    }
    println!("Elapsed: {}ms", now.elapsed().as_millis());
}
