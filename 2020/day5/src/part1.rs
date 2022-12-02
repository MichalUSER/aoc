mod seat;
mod seats_loader;

fn main() {
    //let s = seat::Seat {
        //identifier: vec!['F', 'B', 'F', 'B', 'B', 'F', 'F', 'R', 'L', 'R']
    //};
    //println!("Position: {:?}", s.get_seat());
    //println!("Id: {:?}", s.get_id());
    let seats: Vec<seat::Seat> = seats_loader::get_seats();
    let mut highest_id = 0;
    for s in seats.iter() {
        let sid = s.get_id();
        if sid > highest_id {
            highest_id = sid;
        }
    }
    println!("Seats: {}", seats.len());
    println!("Highest id: {}", highest_id);
}
