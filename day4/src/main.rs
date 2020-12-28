use std::fs;

#[derive(Default)]
struct Passport {
    birth_year: Option<String>,
    issue_year: Option<String>,
    expiration_year: Option<String>,
    height: Option<String>,
    hair_color: Option<String>,
    eye_color: Option<String>,
    passport_id: Option<String>,
    country_id: Option<String>
}

impl Passport {
    fn apply_line(&mut self, line: &str) {
        for pair in line.split(" ") {
            let delim = pair.find(":").expect("Missing colon");
            let key = &pair[..delim];
            let value = &pair[delim+1..];
            if key == "byr" {self.birth_year = Some(value.to_string());}
            else if key == "iyr" {self.issue_year = Some(value.to_string());}
            else if key == "eyr" {self.expiration_year = Some(value.to_string());}
            else if key == "hgt" {self.height = Some(value.to_string());}
            else if key == "hcl" {self.hair_color = Some(value.to_string());}
            else if key == "ecl" {self.eye_color = Some(value.to_string());}
            else if key == "pid" {self.passport_id = Some(value.to_string());}
            else if key == "cid" {self.country_id = Some(value.to_string());}
        }
    }

    fn is_valid_loose(&self) -> bool {
        self.birth_year.is_some() &&
        self.issue_year.is_some() &&
        self.expiration_year.is_some() &&
        self.height.is_some() &&
        self.hair_color.is_some() &&
        self.eye_color.is_some() &&
        self.passport_id.is_some()
    }

    fn is_valid_strict(&self) -> bool {
        if let Some(byr) = &self.birth_year {
            if byr.len() != 4 {
                return false;
            }
            if let Ok(byr_num) = byr.parse::<usize>() {
                if byr_num < 1920 || byr_num > 2002 {
                    return false;
                }
            } else {
                return false;
            }
        } else {
            return false;
        }
        if let Some(iyr) = &self.issue_year {
            if iyr.len() != 4 {
                return false;
            }
            if let Ok(iyr_num) = iyr.parse::<usize>() {
                if iyr_num < 2010 || iyr_num > 2020 {
                    return false;
                }
            } else {
                return false;
            }
        } else {
            return false;
        }
        if let Some(eyr) = &self.expiration_year {
            if eyr.len() != 4 {
                return false;
            }
            if let Ok(eyr_num) = eyr.parse::<usize>() {
                if eyr_num < 2020 || eyr_num > 2030 {
                    return false;
                }
            } else {
                return false;
            }
        } else {
            return false;
        }
        if let Some(hgt) = &self.height {
            if hgt.ends_with("cm") {
                let subst = hgt.splitn(2, "cm").next().expect("Split before unit failed");
                if let Ok(hgt_num) = subst.parse::<usize>() {
                    if hgt_num < 150 || hgt_num > 193 {
                        return false;
                    }
                } else {
                    return false;
                }
            } else if hgt.ends_with("in") {
                let subst = hgt.splitn(2, "in").next().expect("Split before unit failed");
                if let Ok(hgt_num) = subst.parse::<usize>() {
                    if hgt_num < 59 || hgt_num > 76 {
                        return false;
                    }
                } else {
                    return false;
                }
            } else {
                return false;
            }
        } else {
            return false;
        }
        if let Some(hcl) = &self.hair_color {
            if hcl.len() != 7 || !hcl.starts_with("#") {
                return false;
            }
            let mut iter = hcl.chars();
            iter.next();    // skip #
            while let Some(c) = iter.next() {
                if match c {
                    '0'..='9' => false,
                    'a'..='f' => false,
                    _ => true
                } {
                    return false;
                }
            }
        } else {
            return false;
        }
        if let Some(ecl) = &self.eye_color {
            if !vec!["amb","blu","brn","gry","grn","hzl","oth"].contains(&ecl.as_str()) {
                return false;
            }
        } else {
            return false;
        }
        if let Some(pid) = &self.passport_id {
            if pid.len() != 9 {
                return false;
            }
            let mut iter = pid.chars();
            while let Some(c) = iter.next() {
                if match c {
                    '0'..='9' => false,
                    _ => true
                } {
                    return false;
                }
            }
        } else {
            return false;
        }
        true
    }
}

fn main() {
    let file = fs::read_to_string("input.txt").expect("File does not exist");
    let mut passports: Vec<Passport> = vec![];
    let mut maybe_current_passport: Option<Passport> = None;
    for line in file.lines() {
        if line == "" {
            let should_push = maybe_current_passport.is_some();
            if should_push {
                let removal = maybe_current_passport.take();
                passports.push(removal.unwrap());
            }
        } else {
            if maybe_current_passport.is_none() {
                maybe_current_passport = Some(Passport::default());
            }
            maybe_current_passport.as_mut().unwrap().apply_line(line);
        }
    }
    let should_push = maybe_current_passport.is_some();
    if should_push {
        let removal = maybe_current_passport.take();
        passports.push(removal.unwrap());
    }

    let valid_count_loose = passports.iter().map(|p| p.is_valid_loose()).filter(|b| *b).count();
    let valid_count_strict = passports.iter().map(|p| p.is_valid_strict()).filter(|b| *b).count();
    println!("Using the loose criteria, there are {} valid passports.", valid_count_loose);
    println!("Using the strict criteria, there are {} valid passports.", valid_count_strict);
}
