use anyhow::{Ok, Result};

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input.txt")?;

    let result: Vec<(&str, usize)> = input
        .lines()
        .filter_map(|line| {
            let mut splits = line.split(":");
            let service = splits.next()?;
            let users = splits.skip(2).next()?.split(",").count();
            if users <= 1 {
                return None;
            }
            Some((service, users))
        })
        .collect::<Vec<_>>();

    println!("Result : {:#?}", result);

    Ok(())
}
