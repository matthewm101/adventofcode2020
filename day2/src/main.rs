use std::fs;

#[derive(Copy, Clone)]
struct Policy {
    minimum: usize,
    maximum: usize,
    symbol: char
}

struct Case {
    policy: Policy,
    password: String
}

fn check_password_old_policy(policy: Policy, password: &String) -> bool {
    let mut count = 0usize;
    for c in password.chars() {
        if c == policy.symbol {
            count += 1;
            if count > policy.maximum {break;}
        }
    }
    policy.maximum >= count && count >= policy.minimum
}

fn check_password_new_policy(policy: Policy, password: &String) -> bool {
    let char1 = password.chars().nth(policy.minimum).unwrap_or(' ');
    let char2 = password.chars().nth(policy.maximum).unwrap_or(' ');
    (char1 == policy.symbol) ^ (char2 == policy.symbol)
}

fn main() {
    let file = fs::read_to_string("input.txt").expect("File does not exist");
    let mut cases: Vec<Case> = vec![];
    for line in file.lines() {
        let stop1 = line.find("-").expect("Line is missing a dash");
        let stop2 = line.find(" ").expect("Line is missing a space");
        let stop3 = line.find(": ").expect("Line is missing a colon-space");
        let sub1 = String::from(&line[..stop1]);
        let sub2 = String::from(&line[stop1+1..stop2]);
        let sub3 = String::from(&line[stop2+1..stop3]);
        let password = String::from(&line[stop3+1..]);
        let min = sub1.parse::<usize>().expect("Could not parse minimum");
        let max = sub2.parse::<usize>().expect("Could not parse maximum");
        let sym = sub3.chars().next().expect("Could not acquire first character");
        let policy = Policy {minimum: min, maximum: max, symbol: sym};
        cases.push(Case {policy, password});
    }
    let valid_count_old = cases.iter().map(|c| check_password_old_policy(c.policy, &c.password)).filter(|b| *b).count();
    let valid_count_new = cases.iter().map(|c| check_password_new_policy(c.policy, &c.password)).filter(|b| *b).count();
    println!("Number of valid passwords using old policy: {}", valid_count_old);
    println!("Number of valid passwords using new policy: {}", valid_count_new);
}
