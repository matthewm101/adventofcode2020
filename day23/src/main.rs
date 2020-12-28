use std::fs;
use std::collections::HashMap;

// fn step(cups: &Vec<u32>, current_cup: u32) -> (Vec<u32>,u32) {
//     let cc_index = cups.into_iter().position(|c| *c==current_cup).unwrap();
//     let first_cup = cups[(cc_index + 1) % cups.len()];
//     let second_cup = cups[(cc_index + 2) % cups.len()];
//     let third_cup = cups[(cc_index + 3) % cups.len()];
//     let mut dest_cup = current_cup - 1;
//     if dest_cup == 0 {dest_cup = cups.len() as u32;}
//     while dest_cup == first_cup || dest_cup == second_cup || dest_cup == third_cup {
//         dest_cup = dest_cup - 1;
//         if dest_cup == 0 {dest_cup = cups.len() as u32;}
//     }
//     let dest_index = cups.into_iter().position(|c| *c==dest_cup).unwrap();
//     let mut new_cups: Vec<u32> = Vec::with_capacity(cups.len());
//     new_cups.push(dest_cup);
//     new_cups.push(first_cup);
//     new_cups.push(second_cup);
//     new_cups.push(third_cup);
//     let mut readd_index = (dest_index + 1) % cups.len();
//     while readd_index != dest_index {
//         if readd_index == (cc_index + 1) % cups.len() {
//             readd_index = (readd_index + 3) % cups.len();
//         } else {
//             new_cups.push(cups[readd_index]);
//             readd_index = (readd_index + 1) % cups.len();
//         }
//     }
//     let new_cc_index = new_cups.iter().position(|c| *c==current_cup).unwrap();
//     let new_curr_cup = new_cups[(new_cc_index + 1) % cups.len()];
//     return (new_cups, new_curr_cup);
// }

fn fast_step(cups: &mut HashMap<u32,u32>, current_cup: u32) -> u32 {
    let first_cup = *cups.get(&current_cup).unwrap();
    let second_cup = *cups.get(&first_cup).unwrap();
    let third_cup = *cups.get(&second_cup).unwrap();
    let fourth_cup = *cups.get(&third_cup).unwrap();
    cups.insert(current_cup, fourth_cup);   // 0->4, 1->2->3->4, D->A
    let mut dest_cup = current_cup - 1;
    if dest_cup == 0 {dest_cup = cups.len() as u32;}
    while dest_cup == first_cup || dest_cup == second_cup || dest_cup == third_cup {
        dest_cup = dest_cup - 1;
        if dest_cup == 0 {dest_cup = cups.len() as u32;}
    }
    let after_cup = *cups.get(&dest_cup).unwrap();
    cups.insert(dest_cup, first_cup);   // 0->4, D->1->2->3->4
    cups.insert(third_cup, after_cup);  // 0->4, D->1->2->3->A
    return *cups.get(&current_cup).unwrap()
}

fn main() {
    let file = fs::read_to_string("input.txt").expect("File does not exist");
    let cups: Vec<u32> = file.chars().map(|c| c.to_digit(10).unwrap()).collect();
    let mut small_cupmap: HashMap<u32,u32> = HashMap::new();
    for i in 0..cups.len() {
        small_cupmap.insert(cups[i], cups[(i+1) % cups.len()]);
    }
    let mut big_cupmap: HashMap<u32,u32> = small_cupmap.clone();
    for i in ((cups.len() as u32) + 1)..=1000000 {
        big_cupmap.insert(i, i+1);
    }
    big_cupmap.insert(1000000, cups[0]);
    big_cupmap.insert(cups[cups.len()-1], (cups.len() as u32) + 1);

    let mut current_cup = cups[0];
    for _ in 0..100 {
        current_cup = fast_step(&mut small_cupmap, current_cup);
    }
    print!("After 100 steps with {} cups, the cups following the 1 cup are: ", cups.len());
    let mut checked_cup = *small_cupmap.get(&1).unwrap();
    while checked_cup != 1 {
        print!("{}", checked_cup);
        checked_cup = *small_cupmap.get(&checked_cup).unwrap();
    }
    println!("");
    println!("Doing million-cup steps...");
    current_cup = cups[0];
    for i in 0..10000000 {
        current_cup = fast_step(&mut big_cupmap, current_cup);
        if (i+1) % 100000 == 0 {
            print!("{}%...", (i+1) / 100000);
            std::io::Write::flush(&mut std::io::stdout()).unwrap();
        }
    }
    println!("");
    let first_cup_after_1 = *big_cupmap.get(&1).unwrap();
    let second_cup_after_1 = *big_cupmap.get(&first_cup_after_1).unwrap();
    println!("After 10 million steps with a million cups, the cups following the 1 cup are {} and {}.", first_cup_after_1, second_cup_after_1);
    println!("Multiplying these together gives: {}", (first_cup_after_1 as u64) * (second_cup_after_1 as u64));
}
