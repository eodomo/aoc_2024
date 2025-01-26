use anyhow::Result;
use aoc_2b::*;
use std::fs;

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
            //dbg!("{:#}", &report);
            safe_count += 1;
        }
    }

    println!("Number of safe reports: {safe_count}");
    Ok(())
}
