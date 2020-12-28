use std::fs;
use std::collections::HashMap;

type Grammar = HashMap<u64,Vec<Rule>>;
type Memoizer<'a> = HashMap<(&'a str,u64),bool>;

enum Rule {
    Term(char),
    Var(Vec<u64>)
}

fn a_cfg(s: &str, g: &Grammar) -> bool {
    let mut memo = Memoizer::new();
    return a_cfg_sub(s, g, 0, &mut memo);
}

fn a_cfg_sub<'a>(s: &'a str, g: &Grammar, target: u64, memo: &mut Memoizer<'a>) -> bool {
    {if memo.contains_key(&(s,target)) {
        return *memo.get(&(s,target)).unwrap();
    }}
    let mut result = false;
    for rule in g.get(&target).unwrap() {
        match rule {
            Rule::Term(c) => {
                if s.len() == 1 && s.chars().next().unwrap() == *c {
                    result = true;
                    break;
                }
            },
            Rule::Var(vars) => {
                if vars.len() == 1 {
                    if a_cfg_sub(s, g, vars[0], memo) {
                        result = true;
                        break;
                    }
                } else if vars.len() == 2 {
                    for i in 1..s.len() {
                        if a_cfg_sub(&s[..i], g, vars[0], memo) 
                        && a_cfg_sub(&s[i..], g, vars[1], memo) {
                            result = true;
                            break;
                        }
                    }
                } else if vars.len() == 3 {
                    for i in 1..s.len()-1 {
                        for j in i+1..s.len() {
                            if a_cfg_sub(&s[..i], g, vars[0], memo) 
                            && a_cfg_sub(&s[i..j], g, vars[1], memo) 
                            && a_cfg_sub(&s[j..], g, vars[2], memo) {
                                result = true;
                                break;
                            }
                        }
                    }
                } else {
                    panic!("handle more cases");
                }
            }
        }
    }
    memo.insert((s,target), result);
    return result;
}

fn main() {
    let file = fs::read_to_string("input.txt").expect("File does not exist");
    let mut rules: Grammar = Grammar::new();
    let mut lines = file.lines();
    let mut line = lines.next().unwrap();
    while line != "" {
        let mut s1 = line.split(": ");
        let lhs = s1.next().unwrap().parse::<u64>().unwrap();
        if !rules.contains_key(&lhs) {
            rules.insert(lhs, Vec::new());
        }
        let rhs_str: &str = s1.next().unwrap();
        if &rhs_str[0..1] == "\"" {
            rules.get_mut(&lhs).unwrap().push(Rule::Term(rhs_str.chars().nth(1).unwrap()));
        } else {
            let s2 = rhs_str.split(" ");
            let mut tenative_var: Vec<u64> = Vec::new();
            for s in s2 {
                if s == "|" {
                    let clone = tenative_var.clone();
                    rules.get_mut(&lhs).unwrap().push(Rule::Var(clone));
                    tenative_var.clear();
                } else {
                    tenative_var.push(s.parse::<u64>().unwrap());
                }
            }
            rules.get_mut(&lhs).unwrap().push(Rule::Var(tenative_var));
        }
        line = lines.next().unwrap();
    }

    let mut accepted_strings: Vec<&str> = Vec::new();
    let mut rejected_strings: Vec<&str> = Vec::new();
    let all_strings = lines.collect::<Vec<&str>>();
    for i in 0..all_strings.len() {
        if a_cfg(all_strings[i], &rules) {
            accepted_strings.push(all_strings[i]);
            println!("Accepted string {}/{}.", i+1, all_strings.len());
        } else {
            rejected_strings.push(all_strings[i]);
            println!("Rejected string {}/{}.", i+1, all_strings.len());
        }
    }

    println!("With the original rules, {}/{} strings are accepted.", accepted_strings.len(), all_strings.len());

    rules.get_mut(&8).unwrap().push(Rule::Var(vec![42,8]));
    rules.get_mut(&11).unwrap().push(Rule::Var(vec![42,11,31]));

    println!("Rules updated.");
    for i in 0..rejected_strings.len() {
        if a_cfg(rejected_strings[i], &rules) {
            accepted_strings.push(rejected_strings[i]);
            println!("Accepted rejected string {}/{}.", i+1, rejected_strings.len());
        } else {
            println!("Re-rejected rejected string {}/{}.", i+1, rejected_strings.len());
        }
    }

    println!("With the updated rules, {}/{} strings are accepted.", accepted_strings.len(), all_strings.len());
}
