#![allow(unused)]
use anyhow::Result;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::Instant;
use std::{fs::File, ops::RangeInclusive};

const X_OFFSET: isize = 7;
const Y_OFFSET: isize = 7;
const Z_OFFSET: isize = 7;
const W_OFFSET: isize = 7;

#[derive(Debug, Clone, Copy)]
enum CellType {
    Active,
    Inactive,
}

impl CellType {
    fn to_string(&self) -> &str {
        match self {
            CellType::Active => "#",
            CellType::Inactive => ".",
        }
    }
}

type Cells = [[[CellType; 15]; 22]; 22];
type Cells4D = [[[[CellType; 15]; 15]; 22]; 22];

fn set_cell_at(cells: &mut Cells, x: isize, y: isize, z: isize, val: CellType) {
    cells[(x + X_OFFSET) as usize][(y + Y_OFFSET) as usize][(z + Z_OFFSET) as usize] = val;
}

fn set_4d_cell_at(cells: &mut Cells4D, x: isize, y: isize, z: isize, w: isize, val: CellType) {
    cells[(x + X_OFFSET) as usize][(y + Y_OFFSET) as usize][(z + Z_OFFSET) as usize]
        [(w + W_OFFSET) as usize] = val;
}

// start with 8x8x1, with ranges [0, 7], [0, 7], [0, 0]
// 6 rounds can at most expand to 20x20x13, with ranges [-6, 13], [-6, 13], [-6, 6]
// make grid one size larger to not have to deal with boundaries
struct PocketDimension {
    cells: Cells,
}

impl PocketDimension {
    fn from_yx(yx: &Vec<Vec<CellType>>) -> Self {
        let mut cells = [[[CellType::Inactive; 15]; 22]; 22];
        for j in 0..yx.len() {
            for i in 0..yx[0].len() {
                cells[i + X_OFFSET as usize][j + Y_OFFSET as usize][0 + Z_OFFSET as usize] =
                    yx[j][i];
            }
        }
        Self { cells }
    }

    fn get_cell_at(&self, x: isize, y: isize, z: isize) -> CellType {
        self.cells[(x + X_OFFSET) as usize][(y + Y_OFFSET) as usize][(z + Z_OFFSET) as usize]
    }

    fn count_active_neightbours(&self, x: isize, y: isize, z: isize) -> usize {
        let mut count = 0;
        for i in -1..=1 {
            for j in -1..=1 {
                for k in -1..=1 {
                    if i == 0 && j == 0 && k == 0 {
                        continue;
                    }
                    if let CellType::Active = self.get_cell_at(x + i, y + j, z + k) {
                        count += 1;
                    }
                }
            }
        }
        count
    }

    fn step(&mut self) {
        let mut new_cells = self.cells.clone();
        for x in -6..=13 {
            for y in -6..=13 {
                for z in -6..=6 {
                    let active_neighbours = self.count_active_neightbours(x, y, z);
                    match self.get_cell_at(x, y, z) {
                        CellType::Active => match active_neighbours {
                            2..=3 => (),
                            _ => set_cell_at(&mut new_cells, x, y, z, CellType::Inactive),
                        },
                        CellType::Inactive => match active_neighbours {
                            3 => set_cell_at(&mut new_cells, x, y, z, CellType::Active),
                            _ => (),
                        },
                    }
                }
            }
        }
        self.cells = new_cells;
    }

    fn count_active_cells(&self) -> usize {
        self.cells
            .iter()
            .flatten()
            .flatten()
            .filter(|cell| match cell {
                CellType::Active => true,
                _ => false,
            })
            .count()
    }

    fn print_all(&self) {
        self.print(-6..=13, -6..=13, -6..=6);
    }

    fn print(
        &self,
        x_range: RangeInclusive<isize>,
        y_range: RangeInclusive<isize>,
        z_range: RangeInclusive<isize>,
    ) {
        println!("\n grid: ");
        for z in z_range.clone() {
            println!("\nz={}", z);
            for y in y_range.clone() {
                for x in x_range.clone() {
                    print!("{}", self.get_cell_at(x, y, z).to_string());
                }
                print!("\n");
            }
        }
    }
}

struct PocketDimension4D {
    cells: Cells4D,
}

impl PocketDimension4D {
    fn from_yx(yx: &Vec<Vec<CellType>>) -> Self {
        let mut cells = [[[[CellType::Inactive; 15]; 15]; 22]; 22];
        for j in 0..yx.len() {
            for i in 0..yx[0].len() {
                cells[i + X_OFFSET as usize][j + Y_OFFSET as usize][0 + Z_OFFSET as usize]
                    [0 + W_OFFSET as usize] = yx[j][i];
            }
        }
        Self { cells }
    }

    fn get_cell_at(&self, x: isize, y: isize, z: isize, w: isize) -> CellType {
        self.cells[(x + X_OFFSET) as usize][(y + Y_OFFSET) as usize][(z + Z_OFFSET) as usize]
            [(w + W_OFFSET) as usize]
    }

    fn count_active_neightbours(&self, x: isize, y: isize, z: isize, w: isize) -> usize {
        let mut count = 0;
        for i in -1..=1 {
            for j in -1..=1 {
                for k in -1..=1 {
                    for l in -1..=1 {
                        if i == 0 && j == 0 && k == 0 && l == 0 {
                            continue;
                        }
                        if let CellType::Active = self.get_cell_at(x + i, y + j, z + k, w + l) {
                            count += 1;
                        }
                    }
                }
            }
        }
        count
    }

    fn step(&mut self) {
        let mut new_cells = self.cells.clone();
        for x in -6..=13 {
            for y in -6..=13 {
                for z in -6..=6 {
                    for w in -6..=6 {
                        let active_neighbours = self.count_active_neightbours(x, y, z, w);
                        match self.get_cell_at(x, y, z, w) {
                            CellType::Active => match active_neighbours {
                                2..=3 => (),
                                _ => set_4d_cell_at(&mut new_cells, x, y, z, w, CellType::Inactive),
                            },
                            CellType::Inactive => match active_neighbours {
                                3 => set_4d_cell_at(&mut new_cells, x, y, z, w, CellType::Active),
                                _ => (),
                            },
                        }
                    }
                }
            }
        }
        self.cells = new_cells;
    }

    fn count_active_cells(&self) -> usize {
        self.cells
            .iter()
            .flatten()
            .flatten()
            .flatten()
            .filter(|cell| match cell {
                CellType::Active => true,
                _ => false,
            })
            .count()
    }

    fn print_all(&self) {
        self.print(-6..=13, -6..=13, -6..=6, -6..=6);
    }

    fn print(
        &self,
        x_range: RangeInclusive<isize>,
        y_range: RangeInclusive<isize>,
        z_range: RangeInclusive<isize>,
        w_range: RangeInclusive<isize>,
    ) {
        println!("\n grid: ");
        for w in w_range.clone() {
            for z in z_range.clone() {
                println!("\nz={}, w={}", z, w);
                for y in y_range.clone() {
                    for x in x_range.clone() {
                        print!("{}", self.get_cell_at(x, y, z, w).to_string());
                    }
                    print!("\n");
                }
            }
        }
    }
}

fn read_initial_state(filename: &str) -> Result<Vec<Vec<CellType>>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let state: Vec<Vec<CellType>> = reader
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => CellType::Inactive,
                    '#' => CellType::Active,
                    _ => panic!("invalid character {}", c),
                })
                .collect()
        })
        .collect();

    Ok(state)
}

fn main() -> Result<()> {
    let start = Instant::now();

    let initial_state = read_initial_state("input.txt")?;
    let mut pocket_dim = PocketDimension::from_yx(&initial_state);
    for _ in 0..6 {
        pocket_dim.step();
    }
    let result1 = pocket_dim.count_active_cells();
    println!("result1 = {}", result1);

    let mut pocket_dim_4d = PocketDimension4D::from_yx(&initial_state);
    for _ in 0..6 {
        pocket_dim_4d.step();
    }
    let result2 = pocket_dim_4d.count_active_cells();
    println!("result2 = {}", result2);

    println!("Finished in {} us", start.elapsed().as_micros());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_yx() {
        let pd = PocketDimension::from_yx(&vec![vec![CellType::Active]]);
        pd.print(0..=2, 0..=2, 0..=0);
        assert_eq!(pd.count_active_cells(), 1);
    }

    #[test]
    fn example() {
        let initial = read_initial_state("test.txt").unwrap();
        let pd = PocketDimension::from_yx(&initial);
        pd.print(0..=2, 0..=2, 0..=0);
        assert_eq!(pd.count_active_cells(), 5);
    }

    #[test]
    fn example_step_1() {
        let initial = read_initial_state("test.txt").unwrap();
        let mut pd = PocketDimension::from_yx(&initial);
        pd.step();
        pd.print(-1..=3, -1..=3, -2..=2);
        assert_eq!(pd.count_active_cells(), 11);
    }

    #[test]
    fn example_step_2() {
        let initial = read_initial_state("test.txt").unwrap();
        let mut pd = PocketDimension::from_yx(&initial);
        pd.step();
        pd.step();
        pd.print(-2..=4, -2..=4, -2..=2);
        assert_eq!(pd.count_active_cells(), 21);
    }

    #[test]
    fn example_step_3() {
        let initial = read_initial_state("test.txt").unwrap();
        let mut pd = PocketDimension::from_yx(&initial);
        pd.step();
        pd.step();
        pd.step();
        pd.print_all();
        assert_eq!(pd.count_active_cells(), 38);
    }

    #[test]
    fn example_step_6() {
        let initial = read_initial_state("test.txt").unwrap();
        let mut pd = PocketDimension::from_yx(&initial);
        for _ in 0..6 {
            pd.step();
        }
        pd.print_all();
        assert_eq!(pd.count_active_cells(), 112);
    }

    #[test]
    fn example_4d_before_steps() {
        let initial = read_initial_state("test.txt").unwrap();
        let pd = PocketDimension4D::from_yx(&initial);
        pd.print(0..=2, 0..=2, 0..=0, 0..=0);
        assert_eq!(pd.count_active_cells(), 5);
    }

    #[test]
    fn example_4d_step_1() {
        let initial = read_initial_state("test.txt").unwrap();
        let mut pd = PocketDimension4D::from_yx(&initial);
        pd.step();
        pd.print(-1..=3, -1..=3, -1..=1, -1..=1);
        assert_eq!(pd.count_active_cells(), 29);
    }

    #[test]
    fn example_4d_step_2() {
        let initial = read_initial_state("test.txt").unwrap();
        let mut pd = PocketDimension4D::from_yx(&initial);
        pd.step();
        pd.step();
        pd.print(-2..=4, -2..=4, -2..=2, -2..=2);
        assert_eq!(pd.count_active_cells(), 60);
    }

    #[test]
    fn example_4d_step_6() {
        let initial = read_initial_state("test.txt").unwrap();
        let mut pd = PocketDimension4D::from_yx(&initial);
        for _ in 0..6 {
            pd.step();
        }
        pd.print_all();
        assert_eq!(pd.count_active_cells(), 848);
    }
}
