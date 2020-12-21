#![allow(unused)]
use anyhow::Result;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::Instant;

#[derive(Debug, Clone, Copy)]
enum CellType {
    Ground,
    Empty,
    Occupied,
}

#[derive(Debug)]
struct Map {
    cells: Vec<Vec<CellType>>,
    width: usize,
    height: usize,
}

impl Map {
    fn new(cells: Vec<Vec<CellType>>) -> Self {
        let width = cells[0].len();
        let height = cells.len();
        Self {
            cells,
            width,
            height,
        }
    }

    fn count_occupied_neighbours(&self, x: usize, y: usize) -> i32 {
        let mut count = 0;
        let x = x as i32;
        let y = y as i32;
        for i in (x - 1)..(x + 2) {
            if i < 0 || i >= self.width as i32 {
                continue;
            }
            for j in (y - 1)..(y + 2) {
                if j < 0 || j >= self.height as i32 {
                    continue;
                }
                if i == x && j == y {
                    continue;
                }
                if let CellType::Occupied = self.cells[j as usize][i as usize] {
                    count += 1;
                }
            }
        }
        count
    }

    fn ray_trace(&self, x: i32, y: i32, dir_x: i32, dir_y: i32) -> i32 {
        let mut i = x + dir_x;
        let mut j = y + dir_y;
        while i >= 0 && i < self.width as i32 && j >= 0 && j < self.height as i32 {
            match &self.cells[j as usize][i as usize] {
                CellType::Occupied => {
                    return 1;
                }
                CellType::Empty => {
                    return 0;
                }
                CellType::Ground => (),
            }
            i += dir_x;
            j += dir_y;
        }
        0
    }

    fn count_visible_occupied(&self, x: usize, y: usize) -> i32 {
        let mut count = 0;
        let x = x as i32;
        let y = y as i32;

        count += self.ray_trace(x, y, 0, -1);
        count += self.ray_trace(x, y, 1, -1);
        count += self.ray_trace(x, y, 1, 0);
        count += self.ray_trace(x, y, 1, 1);
        count += self.ray_trace(x, y, 0, 1);
        count += self.ray_trace(x, y, -1, 1);
        count += self.ray_trace(x, y, -1, 0);
        count += self.ray_trace(x, y, -1, -1);
        count
    }

    fn count_occupied_seats(&self) -> usize {
        self.cells
            .iter()
            .flatten()
            .filter(|c| match c {
                CellType::Occupied => true,
                _ => false,
            })
            .collect::<Vec<&CellType>>()
            .len()
    }

    fn step(&mut self) -> bool {
        let mut new_cells = self.cells.clone();
        let mut has_changed = false;
        for x in 0..self.width {
            for y in 0..self.height {
                new_cells[y][x] = match (self.cells[y][x], self.count_occupied_neighbours(x, y)) {
                    (CellType::Empty, 0) => {
                        has_changed = true;
                        CellType::Occupied
                    }
                    (CellType::Occupied, 4..=8) => {
                        has_changed = true;
                        CellType::Empty
                    }
                    (typ, _) => typ,
                }
            }
        }
        self.cells = new_cells;
        has_changed
    }

    fn step2(&mut self) -> bool {
        let mut new_cells = self.cells.clone();
        let mut has_changed = false;
        for x in 0..self.width {
            for y in 0..self.height {
                new_cells[y][x] = match (self.cells[y][x], self.count_visible_occupied(x, y)) {
                    (CellType::Empty, 0) => {
                        has_changed = true;
                        CellType::Occupied
                    }
                    (CellType::Occupied, 5..=8) => {
                        has_changed = true;
                        CellType::Empty
                    }
                    (typ, _) => typ,
                }
            }
        }
        self.cells = new_cells;
        has_changed
    }

    fn print(&self) {
        let mut output = String::new();
        for line in &self.cells {
            for cell in line {
                output += match cell {
                    CellType::Empty => "L",
                    CellType::Occupied => "#",
                    CellType::Ground => ".",
                };
            }
            output += "\n";
        }
        println!("{}", output);
    }
}

fn read_map() -> Result<Map> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let cells: Vec<Vec<CellType>> = reader
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| match c {
                    'L' => CellType::Empty,
                    '.' => CellType::Ground,
                    _ => panic!("invalid charcater {}", c),
                })
                .collect::<Vec<CellType>>()
        })
        .collect();

    Ok(Map::new(cells))
}

fn main() -> Result<()> {
    let start = Instant::now();

    let mut map = read_map()?;
    while map.step() {
        // map.print();
    }
    let result1 = map.count_occupied_seats();
    println!("part 1 result: {}", result1);

    let mut map = read_map()?;
    while map.step2() {
        // map.print();
    }
    let result2 = map.count_occupied_seats();
    println!("part 2 result: {}", result2);

    println!("Finished in {} us", start.elapsed().as_micros());
    Ok(())
}
