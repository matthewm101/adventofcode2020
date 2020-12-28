use std::fs;
use std::collections::HashSet;

fn count_neighbors_3d(cubes: &HashSet<(i64,i64,i64)>, coord: (i64,i64,i64)) -> usize {
    let mut count = 0usize;
    for i in -1..=1 {
        for j in -1..=1 {
            for k in -1..=1 {
                if i==0 && j==0 && k==0 {continue;}
                if cubes.contains(&(coord.0+i,coord.1+j,coord.2+k)) {count += 1;}
            }
        }
    }
    return count;
}

fn count_neighbors_4d(cubes: &HashSet<(i64,i64,i64,i64)>, coord: (i64,i64,i64,i64)) -> usize {
    let mut count = 0usize;
    for i in -1..=1 {
        for j in -1..=1 {
            for k in -1..=1 {
                for h in -1..=1 {
                    if i==0 && j==0 && k==0 && h==0 {continue;}
                    if cubes.contains(&(coord.0+i,coord.1+j,coord.2+k,coord.3+h)) {count += 1;}
                }
            }
        }
    }
    return count;
}

fn step_3d(cubes: &HashSet<(i64,i64,i64)>) -> HashSet<(i64,i64,i64)> {
    let mut candidates: HashSet<(i64,i64,i64)> = HashSet::new();
    for (x,y,z) in cubes {
        for i in -1..=1 {
            for j in -1..=1 {
                for k in -1..=1 {
                    candidates.insert((x+i,y+j,z+k));
                }
            }
        }
    }
    let mut next: HashSet<(i64,i64,i64)> = HashSet::new();
    for &cube in &candidates {
        if cubes.contains(&cube) {
            let neighbors = count_neighbors_3d(cubes, cube);
            if neighbors == 2 || neighbors == 3 {
                next.insert(cube);
            }
        } else {
            if count_neighbors_3d(cubes, cube) == 3 {
                next.insert(cube);
            }
        }
    }
    return next;
}

fn step_4d(hypercubes: &HashSet<(i64,i64,i64,i64)>) -> HashSet<(i64,i64,i64,i64)> {
    let mut candidates: HashSet<(i64,i64,i64,i64)> = HashSet::new();
    for (x,y,z,w) in hypercubes {
        for i in -1..=1 {
            for j in -1..=1 {
                for k in -1..=1 {
                    for h in -1..=1 {
                        candidates.insert((x+i,y+j,z+k,w+h));
                    }
                }
            }
        }
    }
    let mut next: HashSet<(i64,i64,i64,i64)> = HashSet::new();
    for &hypercube in &candidates {
        if hypercubes.contains(&hypercube) {
            let neighbors = count_neighbors_4d(hypercubes, hypercube);
            if neighbors == 2 || neighbors == 3 {
                next.insert(hypercube);
            }
        } else {
            if count_neighbors_4d(hypercubes, hypercube) == 3 {
                next.insert(hypercube);
            }
        }
    }
    return next;
}

fn main() {
    let file = fs::read_to_string("input.txt").expect("File does not exist");
    let mut cubes: HashSet<(i64,i64,i64)> = HashSet::new();
    let mut hypercubes: HashSet<(i64,i64,i64,i64)> = HashSet::new();
    let mut x = 0;
    for line in file.lines() {
        let mut y = 0;
        for c in line.chars() {
            if c == '#' {
                cubes.insert((x,y,0));
                hypercubes.insert((x,y,0,0));
            }
            y += 1;
        }
        x += 1;
    }
    for _ in 0..6 {
        cubes = step_3d(&cubes);
        hypercubes = step_4d(&hypercubes);
    }
    println!("After 6 steps, there are {} active cubes.", cubes.len());
    println!("After 6 steps, there are {} active hypercubes.", hypercubes.len());
}
