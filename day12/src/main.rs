use std::fs;

#[derive(Copy,Clone,PartialEq)]
enum Action {North(i64),South(i64),East(i64),West(i64),Left(i64),Right(i64),Forward(i64)}

impl Action {
    fn parse(s: &str) -> Option<Action> {
        if s.len() < 2 {return None;}
        match &s[1..].parse::<i64>() {
            Err(_) => None,
            Ok(num) => {
                match &s[0..1] {
                    "N" => Some(Action::North(*num)),
                    "S" => Some(Action::South(*num)),
                    "E" => Some(Action::East(*num)),
                    "W" => Some(Action::West(*num)),
                    "L" => Some(Action::Left(*num)),
                    "R" => Some(Action::Right(*num)),
                    "F" => Some(Action::Forward(*num)),
                    _ => None
                }
            }
        }
    }
}

#[derive(Copy,Clone)]
struct Ship {
    x: i64,
    y: i64,
    d: i64,
    wx: i64,
    wy: i64
}

impl Ship {
    fn default() -> Ship {
        Ship{x: 0, y: 0, d: 0, wx: 10, wy: 1}
    }

    fn act_wrong_method(&self, action: Action) -> Ship {
        match action {
            Action::North(n) => Ship {y: self.y + n, ..*self},
            Action::South(n) => Ship {y: self.y - n, ..*self},
            Action::East(n) => Ship {x: self.x + n, ..*self},
            Action::West(n) => Ship {x: self.x - n, ..*self},
            Action::Left(n) => Ship {d: (self.d + n).rem_euclid(360), ..*self},
            Action::Right(n) => Ship {d: (self.d - n).rem_euclid(360), ..*self},
            Action::Forward(n) => match self.d {
                0 => Ship {x: self.x + n, ..*self},
                90 => Ship {y: self.y - n, ..*self},
                180 => Ship {x: self.x - n, ..*self},
                270 => Ship {y: self.y + n, ..*self},
                _ => panic!("Illegal direction")
            },
        }
    }

    fn act_right_method(&self, action: Action) -> Ship {
        match action {
            Action::North(n) => Ship {wy: self.wy + n, ..*self},
            Action::South(n) => Ship {wy: self.wy - n, ..*self},
            Action::East(n) => Ship {wx: self.wx + n, ..*self},
            Action::West(n) => Ship {wx: self.wx - n, ..*self},
            Action::Left(n) => match n {
                0 => *self,
                90 => Ship {wx: -self.wy, wy: self.wx, ..*self},
                180 => Ship {wx: -self.wx, wy: -self.wy, ..*self},
                270 => Ship {wx: self.wy, wy: -self.wx, ..*self},
                _ => panic!("Illegal direction")
            },
            Action::Right(n) => match n {
                0 => *self,
                90 => Ship {wx: self.wy, wy: -self.wx, ..*self},
                180 => Ship {wx: -self.wx, wy: -self.wy, ..*self},
                270 => Ship {wx: -self.wy, wy: self.wx, ..*self},
                _ => panic!("Illegal direction")
            },
            Action::Forward(n) => Ship {x: self.x + self.wx * n, y: self.y + self.wy * n, ..*self},
        }
    }
}

fn main() {
    let file = fs::read_to_string("input.txt").expect("File does not exist");
    let mut actions: Vec<Action> = Vec::new();
    for line in file.lines() {
        actions.push(Action::parse(line).unwrap());
    }
    let mut ship = Ship::default();
    for &action in &actions {
        ship = ship.act_wrong_method(action);
    }
    println!("The first action interpretation puts the ship at ({},{}), bearing {}.", ship.x, ship.y, ship.d);
    println!("The ship's Manhattan distance from the origin is {}.", ship.x.abs() + ship.y.abs());

    ship = Ship::default();
    for &action in &actions {
        ship = ship.act_right_method(action);
    }
    println!("The second action interpretation puts the ship at ({},{}), bearing {}.", ship.x, ship.y, ship.d);
    println!("The ship's Manhattan distance from the origin is {}.", ship.x.abs() + ship.y.abs());
}
