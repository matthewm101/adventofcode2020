use std::fs;

fn validate_number(preamble: &[u64], num: u64) -> bool {
    for i in 0..24 {
        for j in (i+1)..25 {
            if num == preamble[i]+preamble[j] {return true;}
        }
    }
    return false;
}

fn find_wrong_number(data: &Vec<u64>) -> u64 {
    for i in 0..(data.len()-25) {
        if !validate_number(&data[i..(i+25)], data[i+25]) {
            return data[i+25];
        }
    }
    return 0;
}

fn find_contiguous_sum(data: &Vec<u64>, sum: u64) -> (usize,usize) {
    let mut rolling_sum = 0u64;
    let mut lower = 0usize; // inclusive
    let mut upper = 0usize; // exclusive
    loop {
        if rolling_sum == sum {break;}
        if rolling_sum > sum {
            rolling_sum -= data[lower];
            lower += 1;
        }
        if rolling_sum < sum {
            if upper == data.len() {panic!("Contiguous sum not found");}
            rolling_sum += data[upper];
            upper += 1;
        }
    }
    return (lower,upper-1);
}

fn main() {
    let file = fs::read_to_string("input.txt").expect("File does not exist");
    let mut data: Vec<u64> = Vec::new();
    for line in file.lines() {
        data.push(line.parse::<u64>().unwrap());
    }
    let weakness = find_wrong_number(&data);
    println!("The first number that breaks the property is {}.", weakness);

    let (lower,upper) = find_contiguous_sum(&data, weakness);
    println!("The set of numbers that sums to {} goes from indices {} to {}.", weakness, lower, upper);

    let max = data[lower..=upper].into_iter().max().unwrap();
    let min = data[lower..=upper].into_iter().min().unwrap();
    println!("The min and max values in this range are {} and {}.", min, max);
    println!("These values sum to {}.", min+max);
}
