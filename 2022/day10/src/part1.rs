fn check_cycle(x: i32, cycle: i32, signal_strength: &mut i32) {
    if cycle >= 20 && cycle <= 220 && (cycle == 20 || (cycle - 20) % 40 == 0) {
        *signal_strength += cycle * x;
        println!("special cycle -  x: {}, cycle: {}, signal_strength: {}", x, cycle, cycle * x);
    }
}

fn main() {
    let mut signal_strength = 0;
    let _answer = include_str!("../input.txt").lines().fold((1, 0), |(x, cycle), l| {
        let option = l.split_once(" ");
        match option {
            Some((_, value)) => {
                let mut cycle2 = cycle;
                for _ in 0..2 {
                    cycle2 += 1;
                    check_cycle(x, cycle2, &mut signal_strength);
                }
                (x + value.parse::<i32>().unwrap(), cycle2)
            },
            _ => {
                check_cycle(x, cycle + 1, &mut signal_strength);
                (x, cycle + 1)
            }
        }
    });
    println!("signal_strength: {:?}", signal_strength);
}
