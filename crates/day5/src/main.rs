use anyhow::Result;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::Instant;

fn read_boarding_passes() -> Result<Vec<usize>> {
    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);

    let seat_ids: Vec<usize> = reader
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            line.char_indices()
                .map(|(i, c)| {
                    let n = match c {
                        'F' => 0,
                        'B' => 1,
                        'L' => 0,
                        'R' => 1,
                        _ => panic!("Invalid character {}", c),
                    };
                    if i < 7 {
                        n << (6 - i)
                    } else {
                        n << (9 - i)
                    }
                })
                .collect::<Vec<usize>>()
        })
        .map(|bits| {
            let row: usize = bits[0..7].iter().sum();
            let column: usize = bits[7..10].iter().sum();
            row * 8 + column
        })
        .collect();

    Ok(seat_ids)
}

fn find_gap(seat_ids: &[usize], max_id: usize) -> usize {
    let mut register = vec![false; max_id + 1];
    for id in seat_ids {
        register[*id] = true;
    }

    let mut prev: Option<usize> = None;
    let mut gap: Option<usize> = None;
    for i in 0..max_id + 1 {
        match (prev, gap, register[i]) {
            (None, None, false) => (),
            (None, None, true) => prev = Some(i),
            (Some(_), None, false) => gap = Some(i),
            (Some(_), None, true) => prev = Some(i),
            (Some(_), Some(_), false) => {
                prev = None;
                gap = None;
            }
            (Some(_), Some(g), true) => return g,
            _ => panic!("Invalid state {:?} {:?} {:?}", prev, gap, register[i]),
        }
    }
    panic!("Gap index not found")
}

fn main() -> Result<()> {
    let start = Instant::now();

    let seat_ids = read_boarding_passes()?;
    let max_id = seat_ids.iter().max().unwrap();

    println!("Highest seat ID: {}", max_id);
    let gap = find_gap(&seat_ids, *max_id);
    println!("gap: {}", gap);

    println!("Finished in {} us", start.elapsed().as_micros());
    Ok(())
}
