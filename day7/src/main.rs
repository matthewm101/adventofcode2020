use std::fs;
use std::collections::{HashMap,HashSet};

// Counts the number of children spawned from the start node, including the start node itself
fn reverse_rules_dfs(reverse_rules: &HashMap<String,HashSet<String>>, start: &str) -> HashSet<String> {
    let mut nodes: HashSet<String> = HashSet::new();
    nodes.insert(start.to_owned());
    if let Some(set) = reverse_rules.get(start) {
        for parent in set {
            nodes.extend(reverse_rules_dfs(reverse_rules, parent));
        }
    }
    return nodes;
}

// Counts the number of children spawned from the start node, considering counts as well, also includes the start
fn rules_counting_dfs(rules: &HashMap<String,HashMap<String,usize>>, start: &str) -> usize {
    let mut sum = 1usize;
    if let Some(set) = rules.get(start) {
        for (child,count) in set {
            sum += count * rules_counting_dfs(rules, child);
        }
    }
    return sum;
}

fn main() {
    let file = fs::read_to_string("input.txt").expect("File does not exist");

    // Maps a bag to the set of bags it contains and their associated sizes
    let mut rules: HashMap<String, HashMap<String,usize>> = HashMap::new();

    // Maps a bag to the set of bags it is contained in
    let mut reverse_rules: HashMap<String,HashSet<String>> = HashMap::new();

    for line in file.lines() {
        let (container,tail) = line.split_at(line.find(" bags contain ").unwrap());
        rules.insert(container.to_owned(),HashMap::new());
        let mut remaining = tail.split_at(14).1;
        loop {
            let (num_str,bag_and_tail) = remaining.split_at(remaining.find(" ").unwrap());
            if num_str == "no" {
                break;
            } else {
                let num: usize = num_str.parse().unwrap();
                let corrected_bag_and_tail = bag_and_tail.split_at(1).1;
                let (bag,tail_yet_again) = corrected_bag_and_tail.split_at(corrected_bag_and_tail.find(" bag").unwrap());
                rules.get_mut(container).unwrap().insert(bag.to_owned(),num);
                if !reverse_rules.contains_key(bag) {
                    reverse_rules.insert(bag.to_owned(), HashSet::new());
                }
                reverse_rules.get_mut(bag).unwrap().insert(container.to_owned());
                if tail_yet_again.starts_with(" bags.") || tail_yet_again.starts_with(" bag.") {
                    break;
                }
                remaining = tail_yet_again.split_at(tail_yet_again.find(", ").unwrap() + 2).1;
            }
        }
    }
    let shiny_gold_containing_ways = reverse_rules_dfs(&reverse_rules, "shiny gold").len() - 1;
    println!("There are {} ways for the shiny gold bag to be contained.", shiny_gold_containing_ways);

    let shiny_gold_contained = rules_counting_dfs(&rules, "shiny gold") - 1;
    println!("There are {} bags contained in the shiny gold bag.", shiny_gold_contained);
}
