fn _fuel_traveled(crabs: &[u64], pos: u64) -> u64 {
    let mut fuel = 0;

    crabs.iter().for_each(|x| {
        fuel += u64::max(*x, pos) - u64::min(*x, pos);
    });

    fuel
}

fn complex_fuel_traveled(crabs: &[u64], pos: u64) -> u64  {
    let mut fuel = 0;

    crabs.iter().for_each(|x| {
        let d = u64::max(*x, pos) - u64::min(*x, pos);
        fuel += d * (d + 1) / 2
    });

    fuel
}

fn main() {
    // "16,1,2,0,4,2,7,1,2,14"
    let crabs: Vec<u64> = include_str!("../input.txt")
        .trim()
        .split(',')
        .map(str::parse)
        .map(Result::unwrap)
        .collect();
    let low = *crabs.iter().min().unwrap();
    let high = *crabs.iter().max().unwrap();
    println!("low: {}\nhigh: {}", low, high);
    let count = (low..high).map(|x| complex_fuel_traveled(&crabs, x)).min();
    println!("count: {:?}", count);
}
