use std::fs;
use std::collections::{BTreeSet,HashMap};

fn count_chains(nums: &BTreeSet<usize>) -> usize {
    let mut tracker: HashMap<usize,usize> = HashMap::new();
    tracker.insert(0,1);
    for n in nums {
        let mut amount = 0;
        if *n >= 1usize && tracker.contains_key(&(n-1)) {amount += tracker[&(n-1)];}
        if *n >= 2usize && tracker.contains_key(&(n-2)) {amount += tracker[&(n-2)];}
        if *n >= 3usize && tracker.contains_key(&(n-3)) {amount += tracker[&(n-3)];}
        tracker.insert(*n,amount);
    }
    let target_output = *nums.iter().max().unwrap();
    return tracker[&target_output];
}

fn main() {
    let file = fs::read_to_string("input.txt").expect("File does not exist");
    let mut nums: BTreeSet<usize> = BTreeSet::new();
    for line in file.lines() {
        nums.insert(line.parse::<usize>().unwrap());
    }
    nums.insert(nums.iter().max().unwrap() + 3);    // the target output

    let mut diffs = [0,0,0];
    let mut prev = 0usize;
    for n in &nums {
        if n-prev == 1 {diffs[0] += 1;}
        else if n-prev == 2 {diffs[1] += 1;}
        else if n-prev == 3 {diffs[2] += 1;}
        prev = *n;
    }
    println!("The number of 1-diffs, 2-diffs, and 3-diffs are {}, {}, and {}.", diffs[0], diffs[1], diffs[2]);
    println!("The 1-diffs times the 3-diffs is {}.", diffs[0]*diffs[2]);

    let n_chains = count_chains(&nums);
    println!("The number of adapter chains is {}.", n_chains);
}
