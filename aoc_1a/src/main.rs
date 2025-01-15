use anyhow::Result;
use std::fs;

fn main() -> Result<()> {
    let contents = fs::read_to_string("./input")?;

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for (i, val) in contents.split_whitespace().enumerate() {
        let converted: i32 = val.parse::<i32>()?;
        if i % 2 == 0 {
            left.push(converted);
        } else {
            right.push(converted);
        }
    }

    left.sort();
    right.sort();

    let mut result = 0;
    for i in 0..left.len() {
        result += left[i].abs_diff(right[i]);
    }

    println!("{result}");
    Ok(())
}
