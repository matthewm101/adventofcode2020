use std::fs;

fn public_key_to_loop_size(public_key: u64) -> u64 {
    let mut loop_size = 0u64;
    let mut current_key = 1u64;
    while current_key != public_key {
        current_key = (7 * current_key) % 20201227;
        loop_size += 1;
    }
    return loop_size;
}

fn make_encryption_key(loop_size: u64, public_key: u64) -> u64 {
    let mut encryption_key = 1;
    for _ in 0..loop_size {
        encryption_key = (public_key * encryption_key) % 20201227;
    }
    return encryption_key;
}

fn main() {
    let file = fs::read_to_string("input.txt").expect("File does not exist");
    let mut lines = file.lines();
    let public1 = lines.next().unwrap().parse::<u64>().unwrap();
    let public2 = lines.next().unwrap().parse::<u64>().unwrap();
    let loop_size1 = public_key_to_loop_size(public1);
    println!("The first private key (loop size) is {}.", loop_size1);
    let loop_size2 = public_key_to_loop_size(public2);
    println!("The second private key (loop size) is {}.", loop_size2);
    let encryption_key1 = make_encryption_key(loop_size1, public2);
    let encryption_key2 = make_encryption_key(loop_size2, public1);
    println!("The encryption keys received by both users are {} and {}.", encryption_key1, encryption_key2);
}
