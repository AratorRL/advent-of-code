use anyhow::Result;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::Instant;

#[derive(Debug, Clone)]
enum MaskBit {
    Zero,
    One,
    Nothing,
}

#[derive(Debug, Clone)]
struct Mask {
    bits: Vec<MaskBit>,
}

impl Mask {
    fn new() -> Self {
        Self { bits: Vec::new() }
    }

    fn from(s: &str) -> Self {
        let bits = s
            .chars()
            .map(|c| match c {
                '0' => MaskBit::Zero,
                '1' => MaskBit::One,
                'X' => MaskBit::Nothing,
                _ => panic!("invalid character: {}", c),
            })
            .rev()
            .collect();
        Self { bits }
    }
}

fn apply_mask(val: u64, mask: &Mask) -> u64 {
    let mut new_val = val;
    for bit in mask.bits.iter().enumerate() {
        match bit {
            (i, MaskBit::Zero) => new_val &= !(1 << i),
            (i, MaskBit::One) => new_val |= 1 << i,
            _ => (),
        }
    }
    new_val
}

fn apply_floating_mask(addr: usize, mask: &Mask) -> Vec<usize> {
    let mut addresses = vec![addr];
    for bit in mask.bits.iter().enumerate() {
        match bit {
            (_, MaskBit::Zero) => (),
            (i, MaskBit::One) => {
                for addr in &mut addresses {
                    *addr |= 1 << i;
                }
            }
            (i, MaskBit::Nothing) => {
                let mut new_addresses = Vec::new();
                // for each address, make a copy with bit i flipped
                for addr in &mut addresses {
                    new_addresses.push(*addr ^ (1 << i));
                }
                addresses.append(&mut new_addresses);
            }
        }
    }
    addresses
}

struct Computer {
    mask: Mask,
    memory: HashMap<usize, u64>,
}

impl Computer {
    fn new() -> Self {
        Self {
            mask: Mask::new(),
            memory: HashMap::new(),
        }
    }

    fn run_program(&mut self, instructions: &[Instruction]) {
        for instr in instructions {
            match instr {
                Instruction::SetMask(mask) => self.mask = mask.clone(),
                Instruction::Store(index, value) => {
                    self.memory.insert(*index, apply_mask(*value, &self.mask));
                }
            };
        }
    }

    fn run_program2(&mut self, instructions: &[Instruction]) {
        for instr in instructions {
            match instr {
                Instruction::SetMask(mask) => self.mask = mask.clone(),
                Instruction::Store(index, value) => {
                    let addresses = apply_floating_mask(*index, &self.mask);
                    for addr in addresses {
                        self.memory.insert(addr, *value);
                    }
                }
            };
        }
    }

    fn sum_memory(&self) -> u64 {
        self.memory.iter().map(|(_, val)| val).sum()
    }
}

enum Instruction {
    SetMask(Mask),
    Store(usize, u64),
}

fn read_input() -> Result<Vec<Instruction>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let instructions = reader
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            if line.starts_with("mask") {
                let mask = Mask::from(line.split_whitespace().last().unwrap());
                Instruction::SetMask(mask)
            } else {
                let parts: Vec<&str> = line.split_whitespace().collect();
                let index: usize = parts[0]
                    .strip_prefix("mem[")
                    .unwrap()
                    .strip_suffix("]")
                    .unwrap()
                    .parse()
                    .unwrap();
                let value: u64 = parts[2].parse().unwrap();
                Instruction::Store(index, value)
            }
        })
        .collect();

    Ok(instructions)
}

fn main() -> Result<()> {
    let start = Instant::now();

    let instructions = read_input()?;
    let mut computer = Computer::new();
    computer.run_program(&instructions);
    let result1 = computer.sum_memory();
    println!("result1 = {}", result1);

    let mut computer = Computer::new();
    computer.run_program2(&instructions);
    let result2 = computer.sum_memory();
    println!("result2 = {}", result2);

    println!("Finished in {} us", start.elapsed().as_micros());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use std::iter::FromIterator;

    #[test]
    fn mask() {
        let mask = Mask::from("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X");
        assert_eq!(apply_mask(11, &mask), 73);

        assert_eq!(apply_mask(101, &mask), 101);

        assert_eq!(apply_mask(0, &mask), 64);
    }

    #[test]
    fn mask2() {
        let mask = Mask::from("000000000000000000000000000000X1001X");
        assert_eq!(
            vec_to_set(apply_floating_mask(42, &mask)),
            vec_to_set(vec![26, 27, 58, 59])
        );

        let mask = Mask::from("00000000000000000000000000000000X0XX");
        assert_eq!(
            vec_to_set(apply_floating_mask(26, &mask)),
            vec_to_set(vec![16, 17, 18, 19, 24, 25, 26, 27])
        );
    }

    fn vec_to_set(v: Vec<usize>) -> HashSet<usize> {
        HashSet::from_iter(v.iter().cloned())
    }
}
