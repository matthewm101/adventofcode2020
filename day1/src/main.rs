use std::fs;
use std::collections::BTreeSet;

fn main() {
    let file = fs::read_to_string("input.txt").expect("File does not exist");
    let mut nums: BTreeSet<u64> = BTreeSet::new();
    for line in file.lines() {
        let num: u64 = line.parse::<u64>().unwrap();
        nums.insert(num);
    }
    for &num in &nums {
        if num <= 2020 && nums.contains(&(2020-num)) {
            println!("Pair found: ({},{}). Product is {}.", num, 2020-num, num * (2020-num));
            break;
        }
    }
    for &num1 in &nums {
        let mut triple_found = false;
        for &num2 in &nums {
            if num1+num2 <= 2020 && nums.contains(&(2020-num1-num2)) {
                println!("Triple found: ({},{},{}). Product is {}.", num1, num2, 2020-num1-num2, num1 * num2 * (2020-num1-num2));
                triple_found = true;
                break;
            }
        }
        if triple_found {break;}
    }
}
