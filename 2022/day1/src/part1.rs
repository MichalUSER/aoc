/*
fn main() {
    let mut largest = 0;
    let mut sum: u64 = 0;
    include_str!("../input.txt").lines().for_each(|l| {
        if l == "" {
            if sum > largest {
                largest = sum;
            }
            sum = 0;
            return;
        }
        sum += l.parse::<u64>().unwrap();
    });
    println!("largest: {}", largest);
}*/
