use std::fs;
use std::collections::{HashSet,HashMap};
use std::cell::RefCell;

type Id = u64;
type EdgeLoc = u64;
type Transform = u64;

#[derive(Default,Clone,PartialEq,Eq,Debug)]
struct Tile {
    id: Id,
    data: [[bool;10];10],
    matches: HashMap<EdgeLoc,Id>
}

impl Tile {
    // Orientation is a number between 0 and 7
    // If the number is even, the edge is read in a direction that goes counterclockwise around the tile.
    // If the number is odd, the edge is read in a direction that goes clockwise around the tile.
    // If the number is 0-1, the right edge is targeted, and if it is 2-3, the top edge is targeted
    // If the number is 4-5, the left edge is targeted, and if it is 6-7, the bottom edge is targeted
    fn get_edge(&self, edge: EdgeLoc) -> [bool;10] {
        let mut result = [false;10];
        match edge {
            0 => {for i in 0..10 {result[i] = self.data[i][9];} return result;},
            1 => {for i in 0..10 {result[i] = self.data[9-i][9];} return result;},
            2 => {for i in 0..10 {result[i] = self.data[0][i];} return result;},
            3 => {for i in 0..10 {result[i] = self.data[0][9-i];} return result;},
            4 => {for i in 0..10 {result[i] = self.data[9-i][0];} return result;},
            5 => {for i in 0..10 {result[i] = self.data[i][0];} return result;},
            6 => {for i in 0..10 {result[i] = self.data[9][9-i];} return result;},
            7 => {for i in 0..10 {result[i] = self.data[9][i];} return result;},
            _ => panic!("Bad orientation")
        }
    }
}

// Given a transform, returns the edge location that gets moved to edgeloc 0 after the transform is applied.
fn inv_transform_right(t: Transform) -> EdgeLoc {
    match t {
        0 => 0,
        1 => 1,
        2 => 6,
        3 => 3,
        4 => 4,
        5 => 5,
        6 => 2,
        7 => 7,
        _ => panic!("Illegal transform")
    }
}

// Given a transform, returns the edge location that gets moved to edgeloc 6 after the transform is applied.
fn inv_transform_down(t: Transform) -> EdgeLoc {
    match t {
        0 => 6,
        1 => 3,
        2 => 4,
        3 => 5,
        4 => 2,
        5 => 7,
        6 => 0,
        7 => 1,
        _ => panic!("Illegal transform")
    }
}

fn pair_tiles(a: &mut Tile, b: &mut Tile) {
    for i in 0..8 {
        for j in 0..8 {
            if a.get_edge(i) == b.get_edge(j) {
                if a.matches.contains_key(&i) {panic!("repeat key");}
                a.matches.insert(i, b.id);
                // b.matches.insert(j, a.id);
            }
        }
    }
}

fn get_right(tile: &Tile, t: Transform) -> Id {
    // println!("{:?}", tile);
    match t {
        0 | 1 => *tile.matches.get(&0).unwrap(),
        3 | 6 => *tile.matches.get(&2).unwrap(),
        4 | 5 => *tile.matches.get(&4).unwrap(),
        2 | 7 => *tile.matches.get(&6).unwrap(),
        _ => panic!("Bad transform")
    }
}

fn get_down(tile: &Tile, t: Transform) -> Id {
    match t {
        6 | 7 => *tile.matches.get(&0).unwrap(),
        1 | 4 => *tile.matches.get(&2).unwrap(),
        2 | 3 => *tile.matches.get(&4).unwrap(),
        0 | 5 => *tile.matches.get(&6).unwrap(),
        _ => panic!("Bad transform")
    }
}

// tile: the tile being resolved.
// left_edge: the edge of the tile to the left.
// The edge is the transformed left tile's position 0 edge.
// As a result, the current tile, after being transformed, must have edge 5 equal left_edge.
fn resolve_right(tile: &Tile, left_edge: [bool;10]) -> Transform {
    for i in 0..8 {
        if tile.get_edge(i) == left_edge {
            return match i {
                0 => 5,
                1 => 4,
                2 => 7,
                3 => 2,
                4 => 1,
                5 => 0,
                6 => 3,
                7 => 6,
                _ => unreachable!("i reached an illegal value somehow")
            }
        }
    }
    unreachable!("None of the tile's edges match left_edge")
}

// tile: the tile being resolved.
// left_edge: the edge of the tile above.
// The edge is the transformed up tile's position 6 edge.
// As a result, the current tile, after being transformed, must have edge 3 equal left_edge.
fn resolve_down(tile: &Tile, up_edge: [bool;10]) -> Transform {
    for i in 0..8 {
        if tile.get_edge(i) == up_edge {
            return match i {
                0 => 3,
                1 => 2,
                2 => 5,
                3 => 0,
                4 => 7,
                5 => 6,
                6 => 1,
                7 => 4,
                _ => unreachable!("iterator reached an illegal value somehow")
            }
        }
    }
    unreachable!("None of the tile's edges match left_edge")
}

// Transforms will be defined similarly to orientations
// Even transforms do no reflections, Odd transforms have an initial vertical flip
// Transforms 2-3 do a 90deg CCW rotation, 4-5 does 180, 6-7 does 270
fn assemble_lattice(tiles: &HashMap<Id,RefCell<Tile>>) -> [[(Id,Transform);12];12] {
    let mut lattice = [[(0,0);12];12];
    // Step 1: find a corner tile
    let mut found_id = 0;
    for (_,t) in tiles {
        let match_count = t.borrow().matches.iter().map(|(_,v)|*v).collect::<HashSet<Id>>().len();
        if match_count == 2 {found_id = t.borrow().id;break;}
    }
    // println!("Found ID: {}", found_id);
    // Step 2: find an orientation that makes the tile fit in the top-left corner
    let top_left_tile = tiles.get(&found_id).unwrap().borrow();
    // println!("{:#?}", *top_left_tile);
    let using_right_edge = top_left_tile.matches.contains_key(&0) || top_left_tile.matches.contains_key(&1);
    let using_top_edge = top_left_tile.matches.contains_key(&2) || top_left_tile.matches.contains_key(&3);
    if using_right_edge {
        if using_top_edge {
            lattice[0][0] = (top_left_tile.id,6);
        } else {
            lattice[0][0] = (top_left_tile.id,0);
        }
    } else {
        if using_top_edge {
            lattice[0][0] = (top_left_tile.id,4);
        } else {
            lattice[0][0] = (top_left_tile.id,2);
        }
    }
    // Step 3: resolve the rest of the orientations
    let mut repeat_checker: HashSet<Id> = HashSet::new();
    let mut repeat_found = false;
    for r in 0..12 { // temp fix later
        for c in 0..12 { // temp fix later
            if r != 0 && c == 0 {
                let (up_id, up_tr) = lattice[r-1][c];
                let up_tile = &tiles.get(&up_id).unwrap().borrow();
                let curr_id = get_down(up_tile, up_tr);
                if repeat_checker.contains(&curr_id) {
                    println!("Repeat detected: {}", curr_id);
                    repeat_found = true;
                    break;
                } else {
                    repeat_checker.insert(curr_id);
                }
                let curr_tile = &tiles.get(&curr_id).unwrap().borrow();
                let edge = up_tile.get_edge(inv_transform_down(up_tr));
                lattice[r][c] = (curr_id,resolve_down(curr_tile, edge));
            } else if c != 0 {
                let (left_id, left_tr) = lattice[r][c-1];
                let left_tile = &tiles.get(&left_id).unwrap().borrow();
                let curr_id = get_right(left_tile, left_tr);
                if repeat_checker.contains(&curr_id) {
                    println!("Repeat detected: {}", curr_id);
                    repeat_found = true;
                    break;
                } else {
                    repeat_checker.insert(curr_id);
                }
                let curr_tile = &tiles.get(&curr_id).unwrap().borrow();
                let edge = left_tile.get_edge(inv_transform_right(left_tr));
                lattice[r][c] = (curr_id,resolve_right(curr_tile, edge));
            }
            // println!("At (r={},c={}), tile {} was placed with transform {}.", r, c, lattice[r][c].0, lattice[r][c].1);
        }
        if repeat_found {break;}
    }
    return lattice;
}

fn transform_tile(data: [[bool;10];10], t: Transform) -> [[bool;10];10] {
    let mut result = [[false;10];10];
    for r in 0..10 {
        for c in 0..10 {
            result[r][c] = match t {
                0 => data[r][c],
                1 => data[9-r][c],
                2 => data[c][9-r],
                3 => data[9-c][9-r],
                4 => data[9-r][9-c],
                5 => data[r][9-c],
                6 => data[9-c][r],
                7 => data[c][r],
                _ => panic!("Illegal transform")
            }
        }
    }
    return result;
}

fn trim_tile(data: [[bool;10];10]) -> [[bool;8];8] {
    let mut result = [[false;8];8];
    for r in 0..8 {
        for c in 0..8 {
            result[r][c] = data[r+1][c+1];
        }
    }
    return result;
}

// fn assemble_debug_image(tiles: &HashMap<Id,RefCell<Tile>>, lattice: [[(u64,u64);12];12]) -> [[bool;120];120] {
//     let mut image = [[false;120];120];
//     for i in 0..12 { // temp fix later
//         for j in 0..12 {
//             let (id,t) = lattice[i][j];
//             let data = tiles.get(&id).and_then(|t| Some(t.borrow().data)).unwrap_or([[false;10];10]);//        .unwrap().borrow().data
//             let transformed_data = transform(data, t);
//             for r in 0..10 {
//                 for c in 0..10 {
//                     image[10*i+r][10*j+c] = transformed_data[r][c];
//                 }
//             }
//         }
//     }
//     return image;
// }

fn transform_image(data: [[bool;96];96], t: Transform) -> [[bool;96];96] {
    let mut result = [[false;96];96];
    for r in 0..96 {
        for c in 0..96 {
            result[r][c] = match t {
                0 => data[r][c],
                1 => data[95-r][c],
                2 => data[c][95-r],
                3 => data[95-c][95-r],
                4 => data[95-r][95-c],
                5 => data[r][95-c],
                6 => data[95-c][r],
                7 => data[c][r],
                _ => panic!("Illegal transform")
            }
        }
    }
    return result;
}

fn assemble_image(tiles: &HashMap<Id,RefCell<Tile>>, lattice: [[(u64,u64);12];12]) -> [[bool;96];96] {
    let mut image = [[false;96];96];
    for i in 0..12 { // temp fix later
        for j in 0..12 {
            let (id,t) = lattice[i][j];
            let data = tiles.get(&id).and_then(|t| Some(t.borrow().data)).unwrap_or([[false;10];10]);//        .unwrap().borrow().data
            let transformed_data = trim_tile(transform_tile(data, t));
            for r in 0..8 {
                for c in 0..8 {
                    image[8*i+r][8*j+c] = transformed_data[r][c];
                }
            }
        }
    }
    return image;
}

// Returns the number of sea monsters, along with the coordinates of all #s that make up monsters.
fn monster_search(image: [[bool;96];96]) -> (usize,HashSet<(usize,usize)>) {
    let monster_offsets = [(0,18),(1,0),(1,5),(1,6),(1,11),(1,12),(1,17),(1,18),(1,19),(2,1),(2,4),(2,7),(2,10),(2,13),(2,16)];
    let mut monster_tiles: HashSet<(usize,usize)> = HashSet::new();
    let mut monster_count = 0usize;
    for r in 0..96-2 {
        for c in 0..96-20 {
            let mut miss = false;
            for (rr,cc) in &monster_offsets {
                if !image[r+rr][c+cc] {
                    miss = true;
                    break;
                }
            }
            if !miss {
                monster_count += 1;
                for (rr,cc) in &monster_offsets {
                    monster_tiles.insert((r+rr,c+cc));
                }
            }
        }
    }
    return (monster_count, monster_tiles);
}

fn count_image_dots(image: [[bool;96];96]) -> usize {
    let mut counter = 0usize;
    for r in 0..96 {
        for c in 0..96 {
            if image[r][c] {counter += 1;}
        }
    }
    return counter;
}

// fn print_debug_image(tiles: &HashMap<Id,RefCell<Tile>>, lattice: [[(u64,u64);12];12]) {
//     let image = assemble_debug_image(tiles, lattice);
//     for r in 0..120 {
//         for c in 0..120 {
//             if image[r][c] {print!("#");} else {print!(".");}
//             if c % 10 == 9 {print!(" ");}
//         }
//         println!();
//         if r % 10 == 9 {println!("");}
//     }
// }

fn print_image(image: [[bool;96];96], monster_tiles: &HashSet<(usize,usize)>) {
    for r in 0..96 {
        for c in 0..96 {
            if image[r][c] {
                if monster_tiles.contains(&(r,c)) {
                    print!("O");
                } else {
                    print!("#");
                }
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

fn main() {
    let mut tiles: HashMap<Id,RefCell<Tile>> = HashMap::new();

    let file = fs::read_to_string("input.txt").expect("File does not exist");
    let mut lines = file.lines().peekable();
    while lines.peek() != None {
        let id = (&lines.next().unwrap()[5..9]).parse::<Id>().unwrap();
        let mut tile = Tile {id: id, data: [[false;10];10], matches: HashMap::new()};
        for i in 0..10 {
            let scanline: &str = lines.next().unwrap();
            for (j,b) in scanline.chars().map(|c| c=='#').enumerate() {
                tile.data[i][j] = b;
            }
        }
        tiles.insert(id,RefCell::new(tile));
        lines.next();
    }

    let ids = tiles.keys().map(|i|*i).collect::<Vec<Id>>();
    for id1 in &ids {
        for id2 in &ids {
            if id1 == id2 {continue;}
            let tile1 = tiles.get(id1).unwrap();
            let tile2 = tiles.get(id2).unwrap();
            pair_tiles(&mut tile1.borrow_mut(), &mut tile2.borrow_mut());
        }
    }

    let mut match_counts: HashMap<usize,Vec<Id>> = HashMap::new();
    for id in &ids {
        let match_count = tiles.get(id).unwrap().borrow().matches.iter().map(|(_,v)|*v).collect::<HashSet<Id>>().len();
        if !match_counts.contains_key(&match_count) {match_counts.insert(match_count, Vec::new());}
        match_counts.get_mut(&match_count).unwrap().push(*id);
    }
    for (ec,v) in &match_counts {
        println!("There are {} tiles with {} matches.", v.len(), *ec);
    }
    println!("The four corner tiles are: {:?}", match_counts.get(&2).unwrap().as_slice());
    println!("The product of these four IDs is {}.", match_counts.get(&2).unwrap().iter().fold(1,|a,b|a*b));

    let lattice = assemble_lattice(&tiles);
    let image = assemble_image(&tiles, lattice);
    for t in 0..8 {
        let t_image = transform_image(image, t);
        let (m_count, m_locs) = monster_search(t_image);
        if m_count > 0 {
            print_image(t_image,&m_locs);
            println!("{} monsters were found.", m_count);
            let n_dots = count_image_dots(t_image);
            println!("{} image dots do not belong to monsters.", n_dots - m_locs.len());
        }
    }
}

