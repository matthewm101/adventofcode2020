use std::fs;
use std::collections::HashSet;

#[derive(Copy,Clone,PartialEq,Eq)]
enum Dir {East, Southeast, Southwest, West, Northwest, Northeast}

fn follow_directions(directions: &Vec<Dir>) -> (i64,i64) {
    let (mut x, mut y) = (0i64, 0i64);
    for &dir in directions {
        let (dx, dy) = match dir {
            Dir::East => (2i64,0i64),
            Dir::Southeast => (1i64,-2i64),
            Dir::Southwest => (-1i64,-2i64),
            Dir::West => (-2i64,0i64),
            Dir::Northwest => (-1i64,2i64),
            Dir::Northeast => (1i64,2i64)
        };
        x += dx;
        y += dy;
    }
    return (x,y);
}

fn flipping_step(tiles: &HashSet<(i64,i64)>) -> HashSet<(i64,i64)> {
    let mut tiles_to_check: HashSet<(i64,i64)> = HashSet::new();
    for &(x,y) in tiles {
        tiles_to_check.insert((x,y));
        tiles_to_check.insert((x+1,y+2));
        tiles_to_check.insert((x+1,y-2));
        tiles_to_check.insert((x-1,y+2));
        tiles_to_check.insert((x-1,y-2));
        tiles_to_check.insert((x+2,y));
        tiles_to_check.insert((x-2,y));
    }
    let mut flipped_tiles: HashSet<(i64,i64)> = HashSet::new();
    for (x,y) in tiles_to_check {
        let mut neighbors = 0usize;
        if tiles.contains(&(x+1,y+2)) {neighbors += 1;}
        if tiles.contains(&(x+1,y-2)) {neighbors += 1;}
        if tiles.contains(&(x-1,y+2)) {neighbors += 1;}
        if tiles.contains(&(x-1,y-2)) {neighbors += 1;}
        if tiles.contains(&(x+2,y)) {neighbors += 1;}
        if tiles.contains(&(x-2,y)) {neighbors += 1;}
        if (tiles.contains(&(x,y)) && (neighbors == 1 || neighbors == 2)) || (!tiles.contains(&(x,y)) && neighbors == 2) {
            flipped_tiles.insert((x,y));
        }
    }
    return flipped_tiles;
}

fn main() {
    let file = fs::read_to_string("input.txt").expect("File does not exist");
    let mut dir_chains: Vec<Vec<Dir>> = Vec::new();
    for line in file.lines() {
        let mut dir_chain: Vec<Dir> = Vec::new();
        let mut chars = line.chars().peekable();
        while chars.peek().is_some() {
            dir_chain.push(
                match chars.next().unwrap() {
                    's' => match chars.next().unwrap() {
                        'e' => Dir::Southeast,
                        'w' => Dir::Southwest,
                        _ => panic!("Invalid character")
                    },
                    'n' => match chars.next().unwrap() {
                        'e' => Dir::Northeast,
                        'w' => Dir::Northwest,
                        _ => panic!("Invalid character")
                    },
                    'e' => Dir::East,
                    'w' => Dir::West,
                    _ => panic!("Invalid character")
                }
            );
        }
        dir_chains.push(dir_chain);
    }

    let mut flipped_tiles: HashSet<(i64,i64)> = HashSet::new();
    for dir_chain in &dir_chains {
        let tile_pos = follow_directions(dir_chain);
        if flipped_tiles.contains(&tile_pos) {
            flipped_tiles.remove(&tile_pos);
        } else {
            flipped_tiles.insert(tile_pos);
        }
    }

    println!("Initially, {} tiles are black.", flipped_tiles.len());

    for i in 0..100 {
        flipped_tiles = flipping_step(&flipped_tiles);
        if i % 10 == 9 {
            print!("{}%...", i+1);
            std::io::Write::flush(&mut std::io::stdout()).unwrap();
        }
    }
    println!("");
    println!("After doing 100 tile flips, {} tiles are black.", flipped_tiles.len());
}
