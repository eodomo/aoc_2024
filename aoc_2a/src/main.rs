use anyhow::Result;
use std::fs;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Direction {
    Increasing,
    Decreasing,
}
#[derive(Debug)]
struct Report {
    levels: Vec<i32>,
    direction: Option<Direction>,
    safe: bool,
}
impl Report {
    fn new() -> Self {
        Report {
            levels: vec![],
            direction: None,
            safe: true,
        }
    }
    fn compare_all_values(&mut self) -> bool {
        let window = self.levels.windows(2);
        for slice in window {
            match self.direction {
                None => {
                    let direction = compare_direction(&slice[0], &slice[1]);
                    self.direction = Some(direction)
                }
                Some(val) => {
                    if compare_direction(&slice[0], &slice[1]) != val {
                        self.safe = false;
                        return false;
                    }
                    if !compare_two_levels(val, &slice[0], &slice[1]) {
                        self.safe = false;
                        return false;
                    }
                }
            }
        }
        true
    }
}

/// true means safe, false means unsafe
fn compare_two_levels(direction: Direction, num0: &i32, num1: &i32) -> bool {
    if num0 == num1 {
        return false;
    }
    if direction == Direction::Increasing {
        if num0 > num1 || *num1 > num0 + 3 {
            false
        } else {
            true
        }
    } else {
        if num0 > num1 || *num0 < num1 - 3 {
            true
        } else {
            false
        }
    }
}

fn compare_direction(num0: &i32, num1: &i32) -> Direction {
    if num1 > num0 {
        Direction::Increasing
    } else {
        Direction::Decreasing
    }
}

fn main() -> Result<()> {
    let contents = fs::read_to_string("./input")?;
    let lines = contents.lines();
    let mut reports: Vec<Report> = Vec::new();

    for line in lines {
        let mut report = Report::new();
        for (_i, level) in line.split_whitespace().enumerate() {
            let converted: i32 = level.parse::<i32>()?;
            report.levels.push(converted);
        }
        reports.push(report);
    }

    let mut safe_count = 0;
    for report in reports.iter_mut() {
        report.compare_all_values();
        if report.safe {
            dbg!("{:#}", &report);
            safe_count += 1;
        }
    }

    println!("{safe_count}");
    Ok(())
}
