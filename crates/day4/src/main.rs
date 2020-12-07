use anyhow::Result;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::Instant;

fn is_valid_year(text: &Option<String>, min: i32, max: i32) -> bool {
    match text {
        None => false,
        Some(t) => {
            if t.len() != 4 {
                return false;
            }
            let year: i32 = match t.parse() {
                Err(_) => return false,
                Ok(y) => y,
            };
            (min..max + 1).contains(&year)
        }
    }
}

fn is_valid_height(text: &Option<String>) -> bool {
    match text {
        None => false,
        Some(t) => {
            if t.contains("cm") {
                let parts: Vec<&str> = t.split("cm").filter(|s| !s.is_empty()).collect();
                if parts.len() == 1 {
                    match parts[0].parse::<i32>() {
                        Err(_) => return false,
                        Ok(value) => return (150..194).contains(&value),
                    }
                }
            } else if t.contains("in") {
                let parts: Vec<&str> = t.split("in").filter(|s| !s.is_empty()).collect();
                if parts.len() == 1 {
                    match parts[0].parse::<i32>() {
                        Err(_) => return false,
                        Ok(value) => return (59..77).contains(&value),
                    }
                }
            }
            false
        }
    }
}

fn is_valid_hair_color(text: &Option<String>) -> bool {
    match text {
        None => false,
        Some(t) => {
            if !t.contains("#") {
                return false;
            }
            let parts: Vec<&str> = t.split("#").filter(|s| !s.is_empty()).collect();
            if parts.len() == 1 {
                println!("parts[0] = {:?}", parts[0]);
                if parts[0].len() == 6 {
                    return parts[0].chars().all(|c| {
                        c.is_ascii_hexdigit() && (c.is_ascii_digit() || c.is_lowercase())
                    });
                }
            }
            false
        }
    }
}

fn is_valid_eye_color(text: &Option<String>) -> bool {
    match text {
        None => return false,
        Some(val) => {
            if !["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&val.as_str()) {
                return false;
            }
            true
        }
    }
}

fn is_valid_pid(text: &Option<String>) -> bool {
    match text {
        None => return false,
        Some(val) => {
            if val.len() != 9 || val.chars().any(|c| !c.is_ascii_digit()) {
                return false;
            }
            true
        }
    }
}

#[derive(Debug)]
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl Passport {
    fn new() -> Self {
        Self {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
        }
    }

    fn from(items: &[(String, String)]) -> Self {
        let mut passport = Self::new();

        for (key, value) in items {
            match key.as_str() {
                "byr" => passport.byr = Some(value.to_owned()),
                "iyr" => passport.iyr = Some(value.to_owned()),
                "eyr" => passport.eyr = Some(value.to_owned()),
                "hgt" => passport.hgt = Some(value.to_owned()),
                "hcl" => passport.hcl = Some(value.to_owned()),
                "ecl" => passport.ecl = Some(value.to_owned()),
                "pid" => passport.pid = Some(value.to_owned()),
                "cid" => passport.cid = Some(value.to_owned()),
                _ => panic!("Invalid key {}", key),
            }
        }

        passport
    }

    fn is_valid(&self) -> bool {
        if !is_valid_year(&self.byr, 1920, 2002) {
            println!("byr not valid for {:?}", self);
            return false;
        }
        if !is_valid_year(&self.iyr, 2010, 2020) {
            println!("iyr not valid for {:?}", self);
            return false;
        }
        if !is_valid_year(&self.eyr, 2020, 2030) {
            println!("eyr not valid for {:?}", self);
            return false;
        }
        if !is_valid_height(&self.hgt) {
            println!("hgt not valid for {:?}", self);
            return false;
        }
        if !is_valid_hair_color(&self.hcl) {
            println!("hcl not valid for {:?}", self);
            return false;
        }
        if !is_valid_eye_color(&self.ecl) {
            println!("ecl not valid for {:?}", self);
            return false;
        }
        if !is_valid_pid(&self.pid) {
            println!("pid not valid for {:?}", self);
            return false;
        }

        true

        // self.byr.is_some()
        //     && self.iyr.is_some()
        //     && self.eyr.is_some()
        //     && self.hgt.is_some()
        //     && self.hcl.is_some()
        //     && self.ecl.is_some()
        //     && self.pid.is_some()
        // && self.cid.is_some()
    }
}

fn read_batch() -> Result<Vec<Passport>> {
    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);

    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    let passports: Vec<Passport> = lines
        .split(|line| line.is_empty())
        .map(|group| {
            group
                .iter()
                .map(|item| {
                    item.split(" ")
                        .map(|s| {
                            let parts: Vec<&str> = s.split(":").collect();
                            (parts[0].to_owned(), parts[1].to_owned())
                        })
                        .collect::<Vec<(String, String)>>()
                })
                .flatten()
                .collect::<Vec<(String, String)>>()
        })
        .map(|group| Passport::from(&group))
        .collect();

    Ok(passports)
}

fn main() -> Result<()> {
    let start = Instant::now();

    let passports = read_batch()?;

    let mut valid_count = 0;
    for passport in passports {
        if passport.is_valid() {
            valid_count += 1;
        }
    }

    println!("Number of valid passports: {}", valid_count);

    println!("Finished in {} us", start.elapsed().as_micros());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn byr() {
        assert_eq!(is_valid_year(&Some(String::from("2002")), 1920, 2002), true);
        assert_eq!(
            is_valid_year(&Some(String::from("2003")), 1920, 2002),
            false
        );
    }

    #[test]
    fn hgt() {
        assert_eq!(is_valid_height(&Some(String::from("60in"))), true);
        assert_eq!(is_valid_height(&Some(String::from("190cm"))), true);
        assert_eq!(is_valid_height(&Some(String::from("190in"))), false);
        assert_eq!(is_valid_height(&Some(String::from("190"))), false);
    }

    #[test]
    fn hcl() {
        assert_eq!(is_valid_hair_color(&Some(String::from("#123abc"))), true);
        assert_eq!(is_valid_hair_color(&Some(String::from("#123abz"))), false);
        assert_eq!(is_valid_hair_color(&Some(String::from("123abc"))), false);
    }

    #[test]
    fn ecl() {
        assert_eq!(is_valid_eye_color(&Some(String::from("brn"))), true);
        assert_eq!(is_valid_eye_color(&Some(String::from("wat"))), false);
    }

    #[test]
    fn pid() {
        assert_eq!(is_valid_pid(&Some(String::from("000000001"))), true);
        assert_eq!(is_valid_pid(&Some(String::from("0123456789"))), false);
    }
}
