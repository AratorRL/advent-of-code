use anyhow::Result;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::Instant;

struct Window {
    numbers: Vec<u64>,
}

impl Window {
    fn new(numbers: &[u64]) -> Self {
        Self {
            numbers: numbers.to_vec(),
        }
    }

    fn find_pair_for(&self, sum: u64) -> Option<(u64, u64)> {
        for i in 0..25 {
            for j in 0..25 {
                if i == j {
                    continue;
                }
                if self.numbers[i] + self.numbers[j] == sum {
                    return Some((self.numbers[i], self.numbers[j]));
                }
            }
        }
        None
    }

    fn advance(&mut self, new: u64) {
        self.numbers.remove(0);
        self.numbers.push(new);
    }
}

fn validate_xmas(numbers: &[u64]) -> u64 {
    let mut window = Window::new(&numbers[0..25]);

    for i in 25..numbers.len() {
        match window.find_pair_for(numbers[i]) {
            Some(_) => (),
            None => return numbers[i],
        }
        window.advance(numbers[i]);
    }
    panic!("no invalid number found")
}

fn find_weakness(numbers: &[u64], sum: u64) -> u64 {
    let mut range: Vec<u64> = Vec::new();
    for num in numbers {
        range.push(*num);
        let curr_sum: u64 = range.iter().sum();
        if curr_sum == sum && range.len() >= 2 {
            break;
        } else if curr_sum > sum {
            let mut new_sum = curr_sum;
            while new_sum > sum {
                new_sum -= range.remove(0);
            }
            if new_sum == sum {
                break;
            }
        }
    }
    println!("range: {:?}", range);
    range.iter().max().unwrap() + range.iter().min().unwrap()
}

fn read_input() -> Result<Vec<u64>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let numbers: Vec<u64> = reader
        .lines()
        .map(|line| line.unwrap())
        .map(|line| line.parse::<u64>().unwrap())
        .collect();

    Ok(numbers)
}

fn main() -> Result<()> {
    let start = Instant::now();

    let numbers = read_input()?;

    let result1 = validate_xmas(&numbers);
    println!("part 1 result: {}", result1);
    println!("part 2 result: {}", find_weakness(&numbers, result1));

    println!("Finished in {} us", start.elapsed().as_micros());
    Ok(())
}
