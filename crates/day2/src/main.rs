use anyhow::Result;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::Instant;

#[derive(Debug)]
struct Policy {
    character: char,
    min: usize,
    max: usize,
}

#[derive(Debug)]
struct Entry {
    policy: Policy,
    password: String,
}

fn read_entries() -> Result<Vec<Entry>> {
    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);

    let entries = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let parts: Vec<&str> = line.split(" ").collect();
            let pol_parts: Vec<&str> = parts[0].split("-").collect();
            let chars: Vec<char> = parts[1].chars().collect();
            let character: char = chars[0];
            let policy = Policy {
                character,
                min: pol_parts[0].parse().unwrap(),
                max: pol_parts[1].parse().unwrap(),
            };
            Entry {
                policy,
                password: String::from(parts[2]),
            }
        })
        .collect();

    Ok(entries)
}

fn check_entries(entries: &[Entry]) {
    let mut correct_count = 0;
    for entry in entries {
        let chars: Vec<char> = entry
            .password
            .chars()
            .filter(|c| c == &entry.policy.character)
            .collect();
        if (entry.policy.min..entry.policy.max + 1).contains(&chars.len()) {
            correct_count += 1;
        }
    }
    println!("Number of correct passwords: {}", correct_count);
}

fn check_entries_v2(entries: &[Entry]) {
    let mut correct_count = 0;
    for entry in entries {
        let chars: Vec<(usize, char)> = entry
            .password
            .char_indices()
            .filter(|(i, c)| {
                (i + 1 == entry.policy.min || i + 1 == entry.policy.max)
                    && c == &entry.policy.character
            })
            .collect();
        if chars.len() == 1 {
            correct_count += 1;
        }
    }
    println!("Number of correct passwords: {}", correct_count);
}

fn main() -> Result<()> {
    let start = Instant::now();

    let entries = read_entries()?;
    println!("{:#?}", &entries);

    check_entries(&entries);
    check_entries_v2(&entries);

    println!("Finished in {} us", start.elapsed().as_micros());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_v2() {
        let entry = Entry {
            policy: Policy {
                character: 'a',
                min: 1,
                max: 3,
            },
            password: String::from("abcde"),
        };
        let chars: Vec<(usize, char)> = entry
            .password
            .char_indices()
            .filter(|(i, c)| {
                println!("checking ({}, {})", i, c);
                (i + 1 == entry.policy.min || i + 1 == entry.policy.max)
                    && c == &entry.policy.character
            })
            .collect();
        assert_eq!(chars.len(), 1);
    }
}
