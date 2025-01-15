use anyhow::Result;
use core::panic;
use std::fs;

enum Direction {
    Increasing,
    Decreasing,
}
struct Level {
    level: Vec<i32>,
    direction: Direction,
}
impl Level {
    fn new() -> Self {
        Level {
            level: vec![],
            direction: Direction::Increasing,
        }
    }
    fn set_direction(&mut self) {
        if self.level.len() < 2 {
            panic!(
                "Level does not contain two or more entries: {:?}",
                self.level
            );
        }
        if self.level[1] > self.level[0] {
            self.direction = Direction::Increasing
        } else {
            self.direction = Direction::Decreasing
        }
    }
    /// set_direction must be run before this
    fn compare_values(&self) -> bool {}
}

fn main() -> Result<()> {
    let contents = fs::read_to_string("./input")?;
    let reports = contents.lines();

    Ok(())
}
