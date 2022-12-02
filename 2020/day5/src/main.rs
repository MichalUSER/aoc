mod seat;
mod seats_loader;

use std::time::Instant;

fn main() {
    let now = Instant::now();
    let seats: Vec<seat::Seat> = seats_loader::get_seats();
    let mut sids: Vec<i32> = Vec::new();
    for i in 0..seats.len() {
        sids.push(seats[i].get_id());
    }
    for sid in sids.iter() {
        if !sids.contains(&(sid-1)) {
            if sid-1 >= 100 && sid-1 <= 700 {
                println!("Seat id: {}", sid-1);
            }
        }
    }
    println!("Elapsed: {}ms", now.elapsed().as_millis());
}
