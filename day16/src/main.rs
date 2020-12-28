use std::fs;
use std::collections::HashSet;

#[derive(Default,Clone,PartialEq,Hash,Eq)]
struct Rule {
    name: String,
    lower1: u64,
    upper1: u64,
    lower2: u64,
    upper2: u64
}

impl std::fmt::Debug for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name.as_str())
    }
}

#[derive(Clone)]
struct Ticket(Vec<u64>);
impl From<Ticket> for Vec<u64> {
    fn from(t: Ticket) -> Vec<u64> {t.0}
}

impl Rule {
    fn parse(line: &str) -> Rule {
        let mut s1 = line.split(": ");
        let name = s1.next().unwrap();
        let mut s2 = s1.next().unwrap().split("-");
        let lower1 = s2.next().unwrap().parse::<u64>().unwrap();
        let mut s3 = s2.next().unwrap().split(" or ");
        let upper1 = s3.next().unwrap().parse::<u64>().unwrap();
        let mut s4 = s3.next().unwrap().split("-");
        let lower2 = s4.next().unwrap().parse::<u64>().unwrap();
        let upper2 = s2.next().unwrap().parse::<u64>().unwrap();
        Rule {name: name.to_owned(), lower1, upper1, lower2, upper2}
    }

    fn check(&self, n: u64) -> bool {
        (self.lower1 <= n && self.upper1 >= n) || (self.lower2 <= n && self.upper2 >= n)
    }
}

fn get_invalid_numbers(ticket: &Ticket, rules: &Vec<Rule>) -> Vec<u64> {
    let mut invalids: Vec<u64> = Vec::new();
    for &n in &ticket.0 {
        let mut meets_rule = false;
        for rule in rules {
            if rule.check(n) {
                meets_rule = true;
                break;
            }
        }
        if !meets_rule {invalids.push(n);}
    }
    return invalids;
}

fn main() {
    let mut rules: Vec<Rule> = Vec::new();
    let mut my_ticket = Ticket(Vec::new());
    let mut scanned_tickets: Vec<Ticket> = Vec::new();
    let mut valid_tickets: Vec<&Ticket> = Vec::new();

    let file = fs::read_to_string("input.txt").expect("File does not exist");
    let mut lines = file.lines();
    let mut next_line = lines.next().unwrap();
    while next_line != "" {
        rules.push(Rule::parse(next_line));
        next_line = lines.next().unwrap();
    }
    lines.next();
    let my_ticket_iter = lines.next().unwrap().split(",").map(|n| n.parse::<u64>().unwrap());
    for n in my_ticket_iter {my_ticket.0.push(n);}
    lines.next();
    lines.next();
    for line in lines {
        let ticket_iter = line.split(",").map(|n| n.parse::<u64>().unwrap());
        let mut ticket = Ticket(Vec::new());
        for n in ticket_iter {ticket.0.push(n);}
        scanned_tickets.push(ticket);
    }
    let num_fields = my_ticket.0.len();

    let mut all_invalids: Vec<u64> = Vec::new();
    for ticket in &scanned_tickets {
        let mut invalids = get_invalid_numbers(ticket, &rules);
        if invalids.len() == 0 {valid_tickets.push(ticket)};
        all_invalids.append(&mut invalids);
    }
    let scanning_error_rate: u64 = all_invalids.iter().sum();
    println!("The scanning error rate is {}.", scanning_error_rate);

    let mut field_rule_sets: Vec<HashSet<usize>> = Vec::new();
    for _ in 0..num_fields {
        field_rule_sets.push((0..rules.len()).collect());
    }
    for ticket in &valid_tickets {
        for i in 0..num_fields {
            let num = ticket.0[i];
            field_rule_sets[i] = field_rule_sets[i].iter().map(|r|*r).filter(|r| rules[*r].check(num)).collect();
        }
    }
    let mut fields_to_check: HashSet<usize> = (0..num_fields).collect();
    while fields_to_check.len() > 0 {
        let ftc_clone = fields_to_check.clone();
        for field_num in ftc_clone {
            if field_rule_sets[field_num].len() == 1 {
                let isolated_rule_num = *field_rule_sets[field_num].iter().next().unwrap();
                println!("{}",rules[isolated_rule_num].name);
                fields_to_check.remove(&field_num);
                for rule_set in &mut field_rule_sets {
                    if rule_set.len() > 1 {
                        rule_set.remove(&isolated_rule_num);
                    }
                }
            }
        }
    }
    let mut product = 1;
    for i in 0..num_fields {
        let name = rules[*field_rule_sets[i].iter().next().unwrap()].name.as_str();
        let num = my_ticket.0[i];
        println!("Your {} is {}.", name, num);
        if name.starts_with("departure") {
            product *= num;
        }
    }
    println!("The product of all 'departure' numbers is {}.", product);
}
