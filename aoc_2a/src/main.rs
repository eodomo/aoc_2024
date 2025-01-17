use anyhow::Result;
use core::panic;
use std::fs;

enum Direction {
    Increasing,
    Decreasing,
}
struct Report {
    levels: Vec<i32>,
    direction: Direction,
    safe: bool,
}
impl Report {
    fn new() -> Self {
        Report {
            levels: vec![],
            direction: Direction::Increasing,
            safe: true,
        }
    }
    fn set_direction(&mut self) {
        if self.levels.len() < 2 {
            panic!(
                "Level does not contain two or more entries: {:?}",
                self.levels
            );
        }
        if self.levels[1] > self.levels[0] {
            self.direction = Direction::Increasing
        } else {
            self.direction = Direction::Decreasing
        }
    }
    /// set_direction must be run before this
    fn compare_values_increasing(&self) -> bool {
        let mut iter = self.levels.iter().peekable();
        for level in iter {
            let next_level = iter.peek();
        }
        true
    }
}

fn main() -> Result<()> {
    let contents = fs::read_to_string("./input")?;
    let reports = contents.lines();

    Ok(())
}
