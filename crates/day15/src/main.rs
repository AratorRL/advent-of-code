use anyhow::Result;
use std::collections::HashMap;
use std::time::Instant;

const INPUT: [u64; 6] = [6, 3, 15, 13, 1, 0];

struct Game {
    last_turn: usize,
    last_spoken: u64,
    last_spoken_at: HashMap<u64, usize>,
}

impl Game {
    fn new(start: Vec<u64>) -> Self {
        let mut last_spoken_at = HashMap::new();

        // start at turn 1
        for (i, n) in start.iter().enumerate() {
            last_spoken_at.insert(*n, i + 1);
        }
        Self {
            last_turn: start.len(),
            last_spoken: *start.last().unwrap(),
            last_spoken_at,
        }
    }

    fn next_turn(&mut self) {
        let next_number: u64 = match self.last_spoken_at.get(&self.last_spoken) {
            Some(index) => (self.last_turn - *index) as u64,
            None => 0,
        };
        self.last_spoken_at.insert(self.last_spoken, self.last_turn);
        self.last_turn += 1;
        self.last_spoken = next_number;
    }

    fn play_until_turn(&mut self, turn: usize) -> u64 {
        while self.last_turn < turn {
            self.next_turn();
        }
        self.last_spoken
    }
}

fn main() -> Result<()> {
    let start = Instant::now();

    let mut game = Game::new(INPUT.to_vec());
    let result1 = game.play_until_turn(2020);
    println!("result1 = {}", result1);

    let mut game = Game::new(INPUT.to_vec());
    let result2 = game.play_until_turn(30_000_000);
    println!("result2 = {}", result2);

    println!("Finished in {} us", start.elapsed().as_micros());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn turn2020() {
        let mut game = Game::new(vec![0, 3, 6]);
        assert_eq!(game.play_until_turn(10), 0);

        let mut game = Game::new(vec![1, 3, 2]);
        assert_eq!(game.play_until_turn(2020), 1);

        let mut game = Game::new(vec![2, 1, 3]);
        assert_eq!(game.play_until_turn(2020), 10);

        let mut game = Game::new(vec![1, 2, 3]);
        assert_eq!(game.play_until_turn(2020), 27);

        let mut game = Game::new(vec![2, 3, 1]);
        assert_eq!(game.play_until_turn(2020), 78);

        let mut game = Game::new(vec![3, 2, 1]);
        assert_eq!(game.play_until_turn(2020), 438);

        let mut game = Game::new(vec![3, 1, 2]);
        assert_eq!(game.play_until_turn(2020), 1836);
    }

    #[test]
    fn turn30000000() {
        let mut game = Game::new(vec![0, 3, 6]);
        assert_eq!(game.play_until_turn(30_000_000), 175594);

        let mut game = Game::new(vec![1, 3, 2]);
        assert_eq!(game.play_until_turn(30_000_000), 2578);

        let mut game = Game::new(vec![2, 1, 3]);
        assert_eq!(game.play_until_turn(30_000_000), 3544142);

        let mut game = Game::new(vec![1, 2, 3]);
        assert_eq!(game.play_until_turn(30_000_000), 261214);

        let mut game = Game::new(vec![2, 3, 1]);
        assert_eq!(game.play_until_turn(30_000_000), 6895259);

        let mut game = Game::new(vec![3, 2, 1]);
        assert_eq!(game.play_until_turn(30_000_000), 18);

        let mut game = Game::new(vec![3, 1, 2]);
        assert_eq!(game.play_until_turn(30_000_000), 362);
    }
}
