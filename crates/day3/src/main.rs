use anyhow::Result;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::Instant;

#[derive(Clone, Debug, Eq, PartialEq)]
enum Location {
    Empty,
    Tree,
}

#[derive(Debug)]
struct Map {
    grid: Vec<Vec<Location>>,
    width: usize,
    height: usize,
}

impl Map {
    fn new(grid: Vec<Vec<Location>>) -> Self {
        let width = grid[0].len();
        let height = grid.len();
        Self {
            grid,
            width,
            height,
        }
    }

    fn get_location(&self, x: usize, y: usize) -> Location {
        let x = x % self.width;
        self.grid[y][x].clone()
    }
}

fn read_map() -> Result<Map> {
    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);

    let grid: Vec<Vec<Location>> = reader
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| match c {
                    '.' => Location::Empty,
                    '#' => Location::Tree,
                    _ => panic!("Unknown input character {}", c),
                })
                .collect()
        })
        .collect();

    let map = Map::new(grid);
    Ok(map)
}

fn drive(map: &Map, slope_x: usize, slope_y: usize) -> i64 {
    let mut x = 0;
    let mut y = 0;
    let mut num_trees = 0;
    while y < map.height {
        if let Location::Tree = map.get_location(x, y) {
            num_trees += 1;
        }
        x += slope_x;
        y += slope_y;
    }
    num_trees
}

fn main() -> Result<()> {
    let start = Instant::now();

    let map = read_map()?;
    let slope_11 = drive(&map, 1, 1);
    let slope_31 = drive(&map, 3, 1);
    let slope_51 = drive(&map, 5, 1);
    let slope_71 = drive(&map, 7, 1);
    let slope_12 = drive(&map, 1, 2);

    let product = slope_11 * slope_31 * slope_51 * slope_71 * slope_12;

    println!("Product: {}", product);

    println!("Finished in {} us", start.elapsed().as_micros());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_location() {
        let map = read_map().unwrap();
        // println!("{:#?}", map.grid[0]);
        println!("{:?}", map.get_location(0, 0));
        assert!(matches!(map.get_location(29, 0), Location::Tree));
        assert!(matches!(map.get_location(30, 0), Location::Empty));
        assert!(matches!(map.get_location(31, 0), Location::Empty));
        assert!(matches!(map.get_location(32, 0), Location::Tree));
    }
}
