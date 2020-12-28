use std::fs;

#[derive(Default)]
struct Forest {
    width: usize,
    height: usize,
    trees: Vec<Vec<bool>>
}

impl Forest {
    fn is_blocked(&self, x: usize, y: usize) -> bool {
        self.trees[y][x % self.width]
    }

    fn count_trees_in_path(&self, x_vel: usize, y_vel: usize) -> usize {
        let mut count = 0usize;
        let mut x = 0usize;
        let mut y = 0usize;
        while y < self.height {
            if self.is_blocked(x, y) {
                count += 1;
            }
            x += x_vel;
            y += y_vel;
            x %= self.width;
        }
        count
    }
}

fn main() {
    let file = fs::read_to_string("input.txt").expect("File does not exist");
    let mut forest: Forest = Forest::default();
    for line in file.lines() {
        let mut row: Vec<bool> = vec![];
        for b in line.chars().map(|c| c == '#') {row.push(b);}
        forest.width = row.len();
        forest.trees.push(row);
        forest.height += 1;
    }
    let r1d1 = forest.count_trees_in_path(1, 1);
    let r3d1 = forest.count_trees_in_path(3, 1);
    let r5d1 = forest.count_trees_in_path(5, 1);
    let r7d1 = forest.count_trees_in_path(7, 1);
    let r1d2 = forest.count_trees_in_path(1, 2);
    let product = r1d1 * r3d1 * r5d1 * r7d1 * r1d2;
    println!("Right 1, down 1: {} trees", r1d1);
    println!("Right 3, down 1: {} trees", r3d1);
    println!("Right 5, down 1: {} trees", r5d1);
    println!("Right 7, down 1: {} trees", r7d1);
    println!("Right 1, down 2: {} trees", r1d2);
    println!("Product: {}", product);
}
