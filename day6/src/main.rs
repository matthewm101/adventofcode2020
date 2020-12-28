use std::fs;
use std::collections::HashSet;

#[derive(Default)]
struct Group {
    persons: Vec<Person>,
    any_yes_responses: HashSet<char>,
    every_yes_responses: HashSet<char>
}

#[derive(Default)]
struct Person {
    yes_responses: HashSet<char>
}

impl Person {
    fn from_responses(responses: &str) -> Person {
        let mut p = Person::default();
        for c in responses.chars() {
            p.yes_responses.insert(c);
        }
        return p;
    }
}

impl Group {
    fn new() -> Group {
        let mut g: Group = Group::default();
        let alphabet = "abcdefghijklmnopqrstuvwxyz";
        g.every_yes_responses.extend(alphabet.chars());
        return g;
    }
    fn add_person(&mut self, person: Person) {
        self.any_yes_responses.extend(&person.yes_responses);
        let intersection = self.every_yes_responses.intersection(&person.yes_responses);
        self.every_yes_responses = intersection.copied().collect();
        self.persons.push(person);
    }
    fn get_n_any_yes(&self) -> usize {
        self.any_yes_responses.len()
    }
    fn get_n_every_yes(&self) -> usize {
        self.every_yes_responses.len() 
    }
}

fn main() {
    let file = fs::read_to_string("input.txt").expect("File does not exist");
    let mut groups: Vec<Group> = vec![];
    let mut current_group: Option<Group> = None;
    for line in file.lines() {
        if line == "" {
            let group_to_add = current_group.take();
            if let Some(g) = group_to_add {
                groups.push(g);
            }
        } else {
            if current_group.is_none() {
                current_group = Some(Group::new());
            }
            if let Some(group) = &mut current_group {
                group.add_person(Person::from_responses(line));
            }
        }
    }
    let group_to_add = current_group.take();
    if let Some(g) = group_to_add {
        groups.push(g);
    }

    let any_yes_sum: usize = groups.iter().map(Group::get_n_any_yes).sum();
    let every_yes_sum: usize = groups.iter().map(Group::get_n_every_yes).sum();
    println!("The number of any-yes responses across all groups is {}.", any_yes_sum);
    println!("The number of every-yes responses across all groups is {}.", every_yes_sum);
}
