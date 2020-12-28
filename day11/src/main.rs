use std::fs;

#[derive(Copy,Clone,PartialEq)]
enum Status {Floor,Empty,Full}

#[derive(Clone)]
struct Ferry {
    seating: Vec<Vec<Status>>,
    width: usize,
    height: usize
}

impl Ferry {
    fn count_full_simple(&self, row: usize, col: usize) -> usize {
        let can_left = col > 0;
        let can_right = col < self.width-1;
        let can_up = row > 0;
        let can_down = row < self.height-1;
        let mut count = 0;
        if can_left && self.seating[row][col-1] == Status::Full {count += 1;}
        if can_right && self.seating[row][col+1] == Status::Full {count += 1;}
        if can_up && self.seating[row-1][col] == Status::Full {count += 1;}
        if can_down && self.seating[row+1][col] == Status::Full {count += 1;}
        if can_left && can_up && self.seating[row-1][col-1] == Status::Full {count += 1;}
        if can_right && can_up && self.seating[row-1][col+1] == Status::Full {count += 1;}
        if can_left && can_down && self.seating[row+1][col-1] == Status::Full {count += 1;}
        if can_right && can_down && self.seating[row+1][col+1] == Status::Full {count += 1;}
        return count;
    }

    fn seek_seat(&self, r: usize, c: usize, rv: i8, cv: i8) -> Status {
        let mut row = r;
        let mut col = c;
        loop {
            if rv < 0 {
                if row == 0 {break;}
                row -= 1;
            }
            if rv > 0 {
                if row == self.height - 1 {break;}
                row += 1;
            }
            if cv < 0 {
                if col == 0 {break;}
                col -= 1;
            }
            if cv > 0 {
                if col == self.width - 1 {break;}
                col += 1;
            }
            if self.seating[row][col] != Status::Floor {return self.seating[row][col];}
        }
        return Status::Floor;
    }

    fn count_full_advanced(&self, row: usize, col: usize) -> usize {
        let mut count = 0usize;
        if self.seek_seat(row, col, -1, 0) == Status::Full {count += 1;}
        if self.seek_seat(row, col, 1, 0) == Status::Full {count += 1;}
        if self.seek_seat(row, col, 0, -1) == Status::Full {count += 1;}
        if self.seek_seat(row, col, 0, 1) == Status::Full {count += 1;}
        if self.seek_seat(row, col, -1, -1) == Status::Full {count += 1;}
        if self.seek_seat(row, col, -1, 1) == Status::Full {count += 1;}
        if self.seek_seat(row, col, 1, -1) == Status::Full {count += 1;}
        if self.seek_seat(row, col, 1, 1) == Status::Full {count += 1;}
        return count;
    }

    fn step(&self, is_adv: bool) -> (Ferry, bool) {
        let mut next: Ferry = self.clone();
        let mut changed = false;
        for row in 0..self.height {
            for col in 0..self.width {
                let next_status = match self.seating[row][col] {
                    Status::Floor => Status::Floor,
                    Status::Empty => {
                        if is_adv {
                            if self.count_full_advanced(row, col) == 0 {Status::Full} else {Status::Empty}
                        } else {
                            if self.count_full_simple(row, col) == 0 {Status::Full} else {Status::Empty}
                        }
                    },
                    Status::Full => {
                        if is_adv {
                            if self.count_full_advanced(row, col) >= 5 {Status::Empty} else {Status::Full}
                        } else {
                            if self.count_full_simple(row, col) >= 4 {Status::Empty} else {Status::Full}
                        }
                    }
                };
                if next_status != self.seating[row][col] {changed = true;}
                next.seating[row][col] = next_status;
            }
        }
        return (next,changed);
    }

    fn simulate(&self, is_adv: bool) -> Ferry {
        let mut next = self.clone();
        let mut changed = true;
        while changed {
            let result = next.step(is_adv);
            next = result.0;
            changed = result.1;
        }
        return next;
    }

    fn count_full_all(&self) -> usize {
        let mut count = 0usize;
        for row in 0..self.height {
            for col in 0..self.width {
                if self.seating[row][col] == Status::Full {count += 1;}
            }
        }
        return count;
    }
}

fn main() {
    let file = fs::read_to_string("input.txt").expect("File does not exist");
    let mut seating: Vec<Vec<Status>> = Vec::new();
    for line in file.lines() {
        let mut row: Vec<Status> = Vec::new();
        for c in line.chars() {
            if c == 'L' {row.push(Status::Empty);} else {row.push(Status::Floor);}
        }
        seating.push(row);
    }
    let h = seating.len();
    let w = seating[0].len();
    let ferry = Ferry {seating: seating, width: w, height: h};
    let ferry_simple = ferry.simulate(false);
    let ferry_adv = ferry.simulate(true);
    println!("Using the basic simulation, there are {} occupied seats.", ferry_simple.count_full_all());
    println!("Using the advanced simulation, there are {} occupied seats.", ferry_adv.count_full_all());
}
