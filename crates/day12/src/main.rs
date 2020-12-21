use anyhow::Result;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::Instant;

#[derive(Debug)]
enum Instruction {
    North(i32),
    East(i32),
    South(i32),
    West(i32),
    Left(i32),
    Right(i32),
    Forward(i32),
}

struct Ship {
    loc_x: i32,
    loc_y: i32,
    orientation: i32,
    waypoint_x: i32,
    waypoint_y: i32,
}

impl Ship {
    fn new() -> Self {
        Self {
            loc_x: 0,
            loc_y: 0,
            orientation: 0,
            waypoint_x: 10,
            waypoint_y: 1,
        }
    }

    fn step(&mut self, instr: &Instruction) {
        match instr {
            Instruction::North(n) => self.loc_y += n,
            Instruction::East(n) => self.loc_x += n,
            Instruction::South(n) => self.loc_y -= n,
            Instruction::West(n) => self.loc_x -= n,
            Instruction::Left(n) => self.orientation = pos_modulo(self.orientation + n, 360),
            Instruction::Right(n) => self.orientation = pos_modulo(self.orientation - n, 360),
            Instruction::Forward(n) => match self.orientation {
                0 => self.loc_x += n,
                90 => self.loc_y += n,
                180 => self.loc_x -= n,
                270 => self.loc_y -= n,
                _ => panic!("unsupported orientation {}", self.orientation),
            },
        }
    }

    fn rotate_waypoint(&mut self, angle: i32) {
        match angle {
            0 => (),
            90 => {
                let old_y = self.waypoint_y;
                self.waypoint_y = self.waypoint_x;
                self.waypoint_x = -old_y;
            }
            180 => {
                self.waypoint_x = -self.waypoint_x;
                self.waypoint_y = -self.waypoint_y;
            }
            270 => {
                let old_y = self.waypoint_y;
                self.waypoint_y = -self.waypoint_x;
                self.waypoint_x = old_y;
            }
            _ => panic!("unsupported orientation {}", angle),
        }
    }

    fn step2(&mut self, instr: &Instruction) {
        match instr {
            Instruction::North(n) => self.waypoint_y += n,
            Instruction::East(n) => self.waypoint_x += n,
            Instruction::South(n) => self.waypoint_y -= n,
            Instruction::West(n) => self.waypoint_x -= n,
            Instruction::Left(n) => self.rotate_waypoint(*n),
            Instruction::Right(n) => self.rotate_waypoint(pos_modulo(-n, 360)),
            Instruction::Forward(n) => {
                self.loc_x += n * self.waypoint_x;
                self.loc_y += n * self.waypoint_y;
            }
        }
    }

    fn follow_instructions(&mut self, instructions: &[Instruction]) {
        for instr in instructions {
            self.step(instr);
        }
    }

    fn follow_instructions2(&mut self, instructions: &[Instruction]) {
        for instr in instructions {
            self.step2(instr);
        }
    }
}

fn pos_modulo(n: i32, d: i32) -> i32 {
    ((n % d) + d) % d
}

fn read_instructions() -> Result<Vec<Instruction>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let instructions: Vec<Instruction> = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let number: i32 = line[1..].parse().unwrap();
            match line.chars().next().unwrap() {
                'N' => Instruction::North(number),
                'E' => Instruction::East(number),
                'S' => Instruction::South(number),
                'W' => Instruction::West(number),
                'L' => Instruction::Left(number),
                'R' => Instruction::Right(number),
                'F' => Instruction::Forward(number),
                _ => panic!("invalid input {}", line),
            }
        })
        .collect();

    Ok(instructions)
}

fn main() -> Result<()> {
    let start = Instant::now();

    let mut ship = Ship::new();
    let instructions = read_instructions()?;
    ship.follow_instructions(&instructions);
    let result1 = i32::abs(ship.loc_x) + i32::abs(ship.loc_y);

    println!("result 1 = {}", result1);

    let mut ship = Ship::new();
    ship.follow_instructions2(&instructions);
    let result2 = i32::abs(ship.loc_x) + i32::abs(ship.loc_y);

    println!("result 2 = {}", result2);

    println!("Finished in {} us", start.elapsed().as_micros());
    Ok(())
}
