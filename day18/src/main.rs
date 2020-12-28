use std::fs;

#[derive(PartialEq, Eq, Copy, Clone)]
enum Item {Val(u64),Add,Mul,Push,Pop}

fn parse_equation(eq: &str) -> Vec<Item> {
    let mut result: Vec<Item> = Vec::new();
    for c in eq.chars() {
        match c {
            '0'..='9' => result.push(Item::Val(c.to_digit(10).unwrap() as u64)),
            '+' => result.push(Item::Add),
            '*' => result.push(Item::Mul),
            '(' => result.push(Item::Push),
            ')' => result.push(Item::Pop),
            _ => ()
        };
    }
    return result;
}

fn eval_eq_basic(eq: &Vec<Item>) -> u64 {
    let mut opstack: Vec<Item> = Vec::new();
    let mut valstack: Vec<u64> = Vec::new();
    opstack.push(Item::Add);
    valstack.push(0);
    for i in eq {
        match i {
            Item::Add => {opstack.push(Item::Add);},
            Item::Mul => {opstack.push(Item::Mul);},
            Item::Push => {
                opstack.push(Item::Add);
                valstack.push(0);
            },
            Item::Pop => {
                let op = opstack.pop().unwrap();
                let rhs = valstack.pop().unwrap();
                let lhs = valstack.pop().unwrap();
                match op {
                    Item::Add => {valstack.push(lhs + rhs);},
                    Item::Mul => {valstack.push(lhs * rhs);},
                    _ => {panic!("Syntax error in equation: no operator before parentheses block");}
                }
            },
            Item::Val(rhs) => {
                let op = opstack.pop().unwrap();
                let lhs = valstack.pop().unwrap();
                match op {
                    Item::Add => {valstack.push(lhs + rhs);},
                    Item::Mul => {valstack.push(lhs * rhs);},
                    _ => {panic!("Syntax error in equation: no operator before parentheses block");}
                }
            }
        }
    }
    return valstack.pop().unwrap();
}

fn eval_eq_advanced(eq: &Vec<Item>) -> u64 {
    let mut opstack: Vec<Item> = Vec::new();
    let mut valstack: Vec<u64> = Vec::new();
    opstack.push(Item::Add);
    valstack.push(0);
    for i in eq {
        match i {
            Item::Add => {opstack.push(Item::Add);},
            Item::Mul => {opstack.push(Item::Mul);},
            Item::Push => {
                opstack.push(Item::Pop);
                opstack.push(Item::Add);
                valstack.push(0);
            },
            Item::Pop => {
                while opstack.pop().unwrap() == Item::Mul {
                    let rhs = valstack.pop().unwrap();
                    let lhs = valstack.pop().unwrap();
                    valstack.push(lhs * rhs);
                }
                let op = opstack.pop().unwrap();
                match op {
                    Item::Add => {
                        let rhs = valstack.pop().unwrap();
                        let lhs = valstack.pop().unwrap();
                        valstack.push(lhs + rhs);
                    },
                    Item::Mul => {
                        opstack.push(Item::Mul);
                    },
                    _ => {panic!("Syntax error in equation: no operator before parentheses block");}
                }
            },
            Item::Val(rhs) => {
                let op = opstack.pop().unwrap();
                match op {
                    Item::Add => {
                        let lhs = valstack.pop().unwrap();
                        valstack.push(lhs + rhs);
                    },
                    Item::Mul => {
                        opstack.push(Item::Mul);
                        valstack.push(*rhs);
                    },
                    _ => {panic!("Syntax error in equation: no operator before parentheses block");}
                }
            }
        }
    }
    while opstack.pop() == Some(Item::Mul) {
        let rhs = valstack.pop().unwrap();
        let lhs = valstack.pop().unwrap();
        valstack.push(lhs * rhs);
    }
    return valstack.pop().unwrap();
}

fn main() {
    let file = fs::read_to_string("input.txt").expect("File does not exist");
    let mut equations: Vec<Vec<Item>> = Vec::new();
    for line in file.lines() {
        equations.push(parse_equation(line));
    }
    
    let mut basic_results: Vec<u64> = Vec::new();
    for eq in &equations {
        basic_results.push(eval_eq_basic(eq));
    }
    println!("The sum of all results is {}.", basic_results.iter().sum::<u64>());

    let mut advanced_results: Vec<u64> = Vec::new();
    for eq in &equations {
        advanced_results.push(eval_eq_advanced(eq));
    }
    println!("The sum of all results is {}.", advanced_results.iter().sum::<u64>());
}
