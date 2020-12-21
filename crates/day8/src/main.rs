use anyhow::Result;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::Instant;

#[derive(Debug, Clone)]
enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

enum Terminate {
    InfiniteLoop(i32),
    EndOfCode(i32),
}

#[derive(Debug, Clone)]
struct Processor {
    acc: i32,
    prog_counter: usize,
}

impl Processor {
    fn new() -> Self {
        Self {
            acc: 0,
            prog_counter: 0,
        }
    }

    fn reset(&mut self) {
        self.acc = 0;
        self.prog_counter = 0;
    }

    fn run(&mut self, instructions: &[Instruction]) -> Terminate {
        let mut executed = vec![false; instructions.len()];
        loop {
            if self.prog_counter >= instructions.len() {
                return Terminate::EndOfCode(self.acc);
            }
            if executed[self.prog_counter] {
                return Terminate::InfiniteLoop(self.acc);
            }
            executed[self.prog_counter] = true;

            match instructions[self.prog_counter] {
                Instruction::Acc(n) => {
                    self.acc += n;
                    self.prog_counter += 1;
                }
                Instruction::Jmp(n) => {
                    self.prog_counter = (self.prog_counter as i32 + n) as usize;
                }
                Instruction::Nop(_) => {
                    self.prog_counter += 1;
                }
            }
        }
    }
}

fn read_input() -> Result<Vec<Instruction>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let instructions: Vec<Instruction> = reader
        .lines()
        .map(|line| line.unwrap())
        .map(
            |line| match &line.split_whitespace().collect::<Vec<&str>>()[..] {
                ["acc", operand] => Instruction::Acc(operand.parse().unwrap()),
                ["jmp", operand] => Instruction::Jmp(operand.parse().unwrap()),
                ["nop", operand] => Instruction::Nop(operand.parse().unwrap()),
                _ => panic!("invalid input"),
            },
        )
        .collect();

    Ok(instructions)
}

fn brute_force(instructions: &mut [Instruction]) -> i32 {
    let mut processor = Processor::new();

    for i in 0..instructions.len() {
        match instructions[i] {
            Instruction::Acc(_) => continue,
            Instruction::Jmp(n) => {
                instructions[i] = Instruction::Nop(n);
                processor.reset();
                match processor.run(instructions) {
                    Terminate::EndOfCode(acc) => return acc,
                    Terminate::InfiniteLoop(_) => (),
                }
                instructions[i] = Instruction::Jmp(n);
            }
            Instruction::Nop(n) => {
                instructions[i] = Instruction::Jmp(n);
                processor.reset();
                match processor.run(instructions) {
                    Terminate::EndOfCode(acc) => return acc,
                    Terminate::InfiniteLoop(_) => (),
                }
                instructions[i] = Instruction::Nop(n);
            }
        }
    }
    panic!("no fix found")
}

fn main() -> Result<()> {
    let start = Instant::now();

    let mut instructions = read_input()?;
    let mut processor = Processor::new();
    if let Terminate::InfiniteLoop(result) = processor.run(&instructions) {
        println!("part 1 result: {}", result);
    }

    let result2 = brute_force(&mut instructions);
    println!("part 2 result: {}", result2);

    println!("Finished in {} us", start.elapsed().as_micros());
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse() {
        assert_eq!("+3".parse::<i32>().unwrap(), 3);
        assert_eq!("-8".parse::<i32>().unwrap(), -8);
    }
}
