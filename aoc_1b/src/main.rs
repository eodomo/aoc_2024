use anyhow::Result;
use std::collections::HashMap;
use std::fs;

fn main() -> Result<()> {
    let contents = fs::read_to_string("./input")?;

    let mut left: Vec<i32> = Vec::new();
    let mut right: HashMap<i32, i32> = HashMap::new();

    for (side, val) in contents.split_whitespace().enumerate() {
        let converted: i32 = val.parse::<i32>()?;
        if side % 2 == 0 {
            left.push(converted);
        } else {
            let count = right.get(&converted);
            let count = match count {
                Some(i) => i,
                None => &0,
            };
            right.insert(converted, count + 1);
        }
    }

    let mut result = 0;
    for i in left.iter() {
        let rightval = right.get(i);
        let rightval = match rightval {
            Some(j) => &j,
            None => &0,
        };
        result += i * rightval;
    }

    println!("{result}");

    Ok(())
}
