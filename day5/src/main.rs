use std::fs;
use std::collections::BTreeSet;

#[derive(Copy,Clone,Default,Debug)]
struct Seat {
    row: usize,
    column: usize
}

impl Seat {
    fn new(pass: &str) -> Seat {
        let parse = pass.chars().map(|c| if c == 'B' || c == 'R' {1} else {0}).fold(0usize, |acc,next| acc*2+next);
        Seat {row: parse >> 3, column: parse & 7}
    }
    
    fn get_id(&self) -> usize {
        self.row * 8 + self.column
    }
}

fn main() {
    let file = fs::read_to_string("input.txt").expect("File does not exist");
    let mut seats: Vec<Seat> = vec![];
    let mut taken_seat_ids: BTreeSet<usize> = BTreeSet::new();
    for line in file.lines() {
        let seat = Seat::new(line);
        taken_seat_ids.insert(seat.get_id());
        seats.push(seat);
    }
    let max = seats.iter().map(|s| s.get_id()).max().expect("No max seat id found");
    println!("The largest seat id is {}.", max);
    let min = seats.iter().map(|s| s.get_id()).min().expect("No max seat id found");
    println!("The smallest seat id is {}.", min);
    for i in min..=max {
        if !taken_seat_ids.contains(&i) {
            println!("The missing seat is {}.", i);
            break;
        }
    }
}
