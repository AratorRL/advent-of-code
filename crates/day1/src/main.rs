use anyhow::Result;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::Instant;

fn find_two_entries(numbers: &[u32]) {
    for (i, v0) in numbers.iter().enumerate() {
        for v1 in &numbers[i + 1..] {
            if v0 + v1 == 2020 {
                println!("values: {}, {}, product = {}", v0, v1, v0 * v1);
                return;
            }
        }
    }
}

fn find_three_entries(numbers: &[u32]) {
    for (i, v0) in numbers.iter().enumerate() {
        for (j, v1) in numbers[i + 1..].iter().enumerate() {
            for v2 in &numbers[j + 1..] {
                if (v0 + v1 + v2) == 2020 {
                    println!("values: {}, {}, {}, product = {}", v0, v1, v2, v0 * v1 * v2);
                    break;
                }
            }
        }
    }
}

fn read_numbers() -> Result<Vec<u32>> {
    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);

    let numbers: Vec<u32> = reader
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();

    Ok(numbers)
}

fn main() -> Result<()> {
    let start = Instant::now();

    let numbers = read_numbers()?;

    find_two_entries(&numbers);
    find_three_entries(&numbers);

    println!("Finished in {} us", start.elapsed().as_micros());
    Ok(())
}
