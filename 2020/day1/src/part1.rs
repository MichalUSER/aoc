// part1

mod numbers;

fn main() {
    let mut checked_numbers: Vec<i32> = Vec::new();

    for num in numbers::get_numbers().into_iter() {
        if checked_numbers.contains(&(2020 - num)) {
            let result = (2020 - num) * num;
            println!("Result: {}", result);
        }
        checked_numbers.push(num);
    }
}
