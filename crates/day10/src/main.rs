use anyhow::Result;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::Instant;

fn get_adapters() -> Result<Vec<u64>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut numbers: Vec<u64> = reader
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();

    let device = numbers.iter().max().unwrap() + 3;
    numbers.push(device);
    numbers.push(0);
    numbers.sort();

    Ok(numbers)
}

fn get_differences(adapters: &[u64]) -> u64 {
    let mut diff1 = 0;
    let mut diff3 = 0;

    for i in 0..adapters.len() - 1 {
        match adapters[i + 1] - adapters[i] {
            1 => diff1 += 1,
            3 => diff3 += 1,
            _ => (),
        }
    }
    diff1 * diff3
}

fn count_arrangements_internal(
    adapters: &[u64],
    index: usize,
    results: &mut HashMap<usize, u64>,
) -> u64 {
    let len = adapters.len();
    if let Some(result) = results.get(&index) {
        return *result;
    }
    match len - index {
        1 => 1,
        _ => {
            let mut count = 0;
            let mut i = index + 1;
            while i < len && adapters[i] - adapters[index] <= 3 {
                count += count_arrangements_internal(&adapters, i, results);
                i += 1;
            }
            results.insert(index, count);
            count
        }
    }
}

fn count_arrangements(adapters: &[u64]) -> u64 {
    let mut results = HashMap::new();
    count_arrangements_internal(adapters, 0, &mut results)
}

fn main() -> Result<()> {
    let start = Instant::now();
    let adapters = get_adapters()?;

    let result1 = get_differences(&adapters);
    println!("part 1 result: {}", result1);

    let result2 = count_arrangements(&adapters);
    println!("part 2 result: {}", result2);

    println!("Finished in {} us", start.elapsed().as_micros());
    Ok(())
}
