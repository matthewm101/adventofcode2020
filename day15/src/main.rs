use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").expect("File does not exist");
    let starting_numbers: Vec<usize> = file.split(",").map(|s| s.parse::<usize>().unwrap()).collect();
    let mut prev_indices_vec = Vec::with_capacity(30000000);
    for _ in 0..30000000 {prev_indices_vec.push(0);}
    let mut prev_indices = prev_indices_vec.into_boxed_slice();
    let mut count = 1usize;
    let mut prev = 0usize;
    let mut curr = 0usize;
    for &n in &starting_numbers {
        prev_indices[n] = count;
        count += 1;
        prev = n;
    }
    while count <= 2020 {
        let next = if prev_indices[curr] == 0 {0} else {count - prev_indices[curr]};
        prev_indices[curr] = count;
        prev = curr;
        curr = next;
        count += 1;
    }
    println!("The 2020th number in the sequence is {}.", prev);
    while count <= 30000000 {
        let next = if prev_indices[curr] == 0 {0} else {count - prev_indices[curr]};
        prev_indices[curr] = count;
        prev = curr;
        curr = next;
        count += 1;
    }
    println!("The 30000000th number in the sequence is {}.", prev);
}
