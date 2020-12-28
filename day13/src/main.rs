use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").expect("File does not exist");
    let mut lines = file.lines();
    let earliest_timestamp = lines.next().unwrap().parse::<u64>().unwrap();
    let mut buses: Vec<(u64,u64)> = Vec::new();
    let mut counter = 0u64;
    for n in lines.next().unwrap().split(",") {
        if n != "x" {
            buses.push((n.parse::<u64>().unwrap(), counter));
        }
        counter += 1;
    }
    let mut best_bus = 0u64;
    let mut best_waiting_time = u64::MAX;
    for (bus,_) in &buses {
        let waiting_time = if earliest_timestamp % bus == 0 {0} else {(earliest_timestamp/bus+1)*bus-earliest_timestamp};
        if waiting_time < best_waiting_time {
            best_waiting_time = waiting_time;
            best_bus = *bus;
        }
    }
    println!("The best bus is {}. The waiting time will be {}.", best_bus, best_waiting_time);
    println!("These numbers multiplied together is {}.", best_bus * best_waiting_time);

    let mut scaling_factor = 1u64;
    let mut special_time = 0u64;
    for (bus, offset) in &buses {
        print!("New rule added: t+{} mod {} == 0. ", offset, bus);
        while (special_time + offset) % bus != 0 {special_time += scaling_factor;}
        println!("Updated t value: {}.", special_time);
        scaling_factor *= bus;
    }
    println!("The t value that fulfills all the requirements is {}.", special_time);
}
