use anyhow::Result;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::Instant;

fn read_input() -> Result<(u64, Vec<u64>)> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut lines = reader.lines();
    let now: u64 = lines.next().unwrap()?.parse()?;
    let buses: Vec<u64> = lines
        .next()
        .unwrap()?
        .split(",")
        .filter(|s| s != &"x")
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    Ok((now, buses))
}

fn read_input2() -> Result<Vec<Option<u64>>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut lines = reader.lines();
    let _: u64 = lines.next().unwrap()?.parse()?;
    let buses: Vec<Option<u64>> = lines
        .next()
        .unwrap()?
        .split(",")
        .map(|s| match s {
            "x" => None,
            n => Some(n.parse::<u64>().unwrap()),
        })
        .collect();

    Ok(buses)
}

fn part1(now: u64, buses: Vec<u64>) -> u64 {
    let (first_bus_id, first_bus_time) = buses
        .iter()
        .map(|id| (id, id - (now % id)))
        .min_by(|(_, n1), (_, n2)| n1.cmp(n2))
        .unwrap();

    first_bus_id * first_bus_time
}

// Find the multiplicative inverse of a modulo n.
fn mod_inverse(a: u64, n: u64) -> u64 {
    let mut t: i64 = 0;
    let mut r: i64 = n as i64;
    let mut t_new: i64 = 1;
    let mut r_new: i64 = a as i64;
    while r_new != 0 {
        let q = r / r_new;
        let t2 = t;
        t = t_new;
        t_new = t2 - q * t_new;
        let r2 = r;
        r = r_new;
        r_new = r2 - q * r_new;
    }
    if r > 1 {
        panic!("a does not have an inverse")
    }
    if t < 0 {
        t += n as i64;
    }
    t as u64
}

/// Find the lowest c1 such that there is a k with (k == a + c1*m) and (k == b mod n)
fn linear_eq_mod(a: u64, m: u64, b: u64, n: u64) -> u64 {
    // k = c1 * m + a.
    // k = c2 * n + b.
    // -> c1 = (b - a) * m^-1 mod n.
    let m_inv = mod_inverse(m, n);
    let diff = (b as i64 - a as i64).rem_euclid(n as i64) as u64;
    let c1 = (diff * m_inv) % n;
    c1
}

fn part2(buses: Vec<Option<u64>>) -> u64 {
    let constraints: Vec<(u64, u64)> = buses
        .iter()
        .enumerate()
        .filter(|(_, b)| match b {
            Some(_) => true,
            None => false,
        })
        .map(|(i, b)| (i as u64, b.unwrap()))
        .collect();
    let (max_index, max_id) = *constraints
        .iter()
        .max_by(|(_, id1), (_, id2)| id1.cmp(id2))
        .unwrap();

    // Start with a timestamp t such that the highest ID is satisfied.
    let t: u64 = max_id - max_index;

    // Given this t, check if and how much t must be increased for each of the other IDs.
    // An equation item (ID, a) means that to satisfy ID, t must be increased by (a mod ID).
    let mut equations: Vec<(u64, u64)> = Vec::new();
    for (i, id) in &constraints {
        let offset = (t + i) % id;
        if offset != 0 {
            equations.push((*id, id - offset));
        }
    }

    // We need to increase t by some delta D.
    // D must be a multiple of max_id, i.e. k * max_id for some k.
    // Below we figure out a suitable k.
    // Each entry in `equations` gives a constraint on k in the form of a linear equation.
    // `k` can itself be represented as `a mod m`, stored here as a tuple (a, m).
    // We update the tuple while iterating over the constraints, starting with a None value.
    let mut k: Option<(u64, u64)> = None;
    for (id, delay) in equations {
        let c1 = linear_eq_mod(0, max_id, delay, id);
        k = match k {
            None => Some((c1, id)),
            Some((a, m)) => {
                let c1 = linear_eq_mod(a, m, c1, id);
                // c1 is modulo id
                Some((c1 * m + a, id * m))
            }
        }
    }
    let (k, _) = k.unwrap(); // ignore final modulo
    t + k * max_id
}

fn main() -> Result<()> {
    let start = Instant::now();

    let (now, buses) = read_input()?;
    let result1 = part1(now, buses);
    println!("result 1 = {}", result1);

    let buses = read_input2()?;
    let result2 = part2(buses);
    println!("result 2 = {}", result2);

    println!("Finished in {} us", start.elapsed().as_micros());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn euclidean() {
        assert_eq!(mod_inverse(3, 10), 7);
        assert_eq!(mod_inverse(907, 653), 18);
        assert_eq!((mod_inverse(907, 653) * 368) % 653, 94);
        assert_eq!((mod_inverse(907, 41) * 36) % 41, 40);
        assert_eq!(mod_inverse(653, 41), 27);
        assert_eq!((mod_inverse(653, 41) * 28) % 41, 18);
        assert_eq!(linear_eq_mod(0, 907, 368, 653), 94);
        assert_eq!(linear_eq_mod(94, 653, 40, 41), 18);
    }
}
