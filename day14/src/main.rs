use std::fs;
use std::collections::BTreeMap;

fn main() {
    let file = fs::read_to_string("input.txt").expect("File does not exist");
    let mut memory: BTreeMap<u64,u64> = BTreeMap::new();
    let mut set_mask = 0u64;
    let mut clear_mask = 0xFFFFFFFFFu64;
    for line in file.lines() {
        if line.starts_with("mask = ") {
            let substr = &line["mask = ".len()..];
            set_mask = 0;
            clear_mask = 0;
            for c in substr.chars() {
                match c {
                    '1' => {
                        set_mask = (set_mask << 1) + 1;
                        clear_mask = (clear_mask << 1) + 1;
                    },
                    '0' => {
                        set_mask = set_mask << 1;
                        clear_mask = clear_mask << 1;
                    },
                    _ => {
                        set_mask = set_mask << 1;
                        clear_mask = (clear_mask << 1) + 1;
                    }
                }
            }
        }
        if line.starts_with("mem[") {
            let substr = &line["mem[".len()..];
            let mut splitter = substr.split("] = ");
            let address = splitter.next().unwrap().parse::<u64>().unwrap();
            let value = splitter.next().unwrap().parse::<u64>().unwrap();
            memory.insert(address, (value & clear_mask) | set_mask);
        }
    }

    let mut sum = 0u64;
    for (_,v) in &memory {
        sum += v;
    }
    println!("Using version 1, the sum of all values in memory is {}.", sum);

    let mut set_masks: Vec<u64> = Vec::new();
    let mut clear_masks: Vec<u64> = Vec::new();
    memory.clear();

    for line in file.lines() {
        if line.starts_with("mask = ") {
            let substr = &line["mask = ".len()..];
            set_masks.clear();
            set_masks.push(0);
            clear_masks.clear();
            clear_masks.push(0);
            for c in substr.chars() {
                match c {
                    '1' => {
                        set_masks = set_masks.iter().map(|n| (n<<1)+1).collect();
                        clear_masks = clear_masks.iter().map(|n| (n<<1)+1).collect();
                    },
                    '0' => {
                        set_masks = set_masks.iter().map(|n| n<<1).collect();
                        clear_masks = clear_masks.iter().map(|n| (n<<1)+1).collect();
                    },
                    _ => {
                        set_masks = set_masks.iter().map(|n| (n<<1)+1).chain(set_masks.iter().map(|n| n<<1)).collect();
                        clear_masks = clear_masks.iter().map(|n| (n<<1)+1).chain(clear_masks.iter().map(|n| n<<1)).collect();
                    }
                }
            }
        }
        if line.starts_with("mem[") {
            let substr = &line["mem[".len()..];
            let mut splitter = substr.split("] = ");
            let address = splitter.next().unwrap().parse::<u64>().unwrap();
            let value = splitter.next().unwrap().parse::<u64>().unwrap();
            for (s,c) in set_masks.iter().zip(clear_masks.iter()) {
                memory.insert((address & c) | s, value);
            }
        }
    }
    
    sum = 0u64;
    for (_,v) in &memory {
        sum += v;
    }
    println!("Using version 2, the sum of all values in memory is {}.", sum);
}
