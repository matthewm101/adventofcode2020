use std::fs;
use std::collections::{HashSet,LinkedList};

fn simulate_combat(p1: &LinkedList<usize>, p2: &LinkedList<usize>) -> (bool,LinkedList<usize>) {
    let mut player1 = p1.clone();
    let mut player2 = p2.clone();
    let mut repeat_tracker: HashSet<Vec<usize>> = HashSet::new();
    let mut repeat_found = false;
    while player1.len() > 0 && player2.len() > 0 {
        let state = compress_state(&player1,&player2);
        if repeat_tracker.contains(&state) {
            repeat_found = true;
            break;
        }
        repeat_tracker.insert(state);
        let card1 = player1.pop_front().unwrap();
        let card2 = player2.pop_front().unwrap();
        if card1 > card2 {
            player1.push_back(card1);
            player1.push_back(card2);
        } else {
            player2.push_back(card2);
            player2.push_back(card1);
        }
    }
    if repeat_found || player1.len() > 0 {
        return (true,player1);
    } else {
        return (false,player2);
    }
}

fn compress_state(p1: &LinkedList<usize>, p2: &LinkedList<usize>) -> Vec<usize> {
    let mut v = Vec::new();
    v.append(&mut p1.clone().into_iter().collect());
    v.push(0);
    v.append(&mut p2.clone().into_iter().collect());
    return v;
}

// Returns whether player 1 won, and the hand of the winner
fn simulate_recursive_combat(p1: LinkedList<usize>, p2: LinkedList<usize>) -> (bool,LinkedList<usize>) {
    // let initial_state = compress_state(p1, p2);
    // if memo.contains_key(&initial_state) {
    //     return (*memo.get(&initial_state).unwrap(),p1.clone());
    // }
    let mut player1 = p1.clone();
    let mut player2 = p2.clone();
    let mut repeat_tracker: HashSet<Vec<usize>> = HashSet::new();
    let mut repeat_found = false;
    while player1.len() > 0 && player2.len() > 0 {
        let state = compress_state(&player1,&player2);
        if repeat_tracker.contains(&state) {
            repeat_found = true;
            break;
        }
        repeat_tracker.insert(state);
        let card1 = player1.pop_front().unwrap();
        let card2 = player2.pop_front().unwrap();
        let result = if player1.len() >= card1 && player2.len() >= card2 {
            let mut clone1 = player1.clone();
            let mut clone2 = player2.clone();
            while clone1.len() > card1 {clone1.pop_back();}
            while clone2.len() > card2 {clone2.pop_back();}
            simulate_recursive_combat(clone1, clone2).0
        } else {
            card1 > card2
        };
        if result {
            player1.push_back(card1);
            player1.push_back(card2);
        } else {
            player2.push_back(card2);
            player2.push_back(card1);
        }
    }
    if repeat_found || player1.len() > 0 {
        // memo.insert(initial_state, true);
        return (true,player1);
    } else {
        // memo.insert(initial_state, false);
        return (false,player2);
    }
}

fn calculate_score(cards: &LinkedList<usize>) -> usize {
    let mut multiplier = cards.len() as usize;
    let mut score = 0usize;
    for card in cards {
        score += multiplier * card;
        multiplier -= 1;
    }
    return score;
}

fn main() {
    let mut player1_cards: LinkedList<usize> = LinkedList::new();
    let mut player2_cards: LinkedList<usize> = LinkedList::new();

    let file = fs::read_to_string("input.txt").expect("File does not exist");
    let mut lines = file.lines().peekable();
    lines.next();
    while lines.peek().unwrap().len() > 0 {
        player1_cards.push_back(lines.next().unwrap().parse::<usize>().unwrap());
    }
    lines.next();
    lines.next();
    for line in lines {
        player2_cards.push_back(line.parse::<usize>().unwrap());
    }

    let (c_winner,c_final_hand) = simulate_combat(&player1_cards,&player2_cards);
    let c_winner_string = if c_winner {"Player 1"} else {"Player 2"};
    let c_score = calculate_score(&c_final_hand);
    println!("After simulating Combat, {} won with a score of {}.", c_winner_string, c_score);

    // let mut memo = HashMap::new();
    let (rc_winner,rc_final_hand) = simulate_recursive_combat(player1_cards.clone(),player2_cards.clone());
    let rc_winner_string = if rc_winner {"Player 1"} else {"Player 2"};
    let rc_score = calculate_score(&rc_final_hand);
    println!("After simulating Recursive Combat, {} won with a score of {}.", rc_winner_string, rc_score);
}
