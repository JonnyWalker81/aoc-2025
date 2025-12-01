use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let _lines: Vec<&str> = input.trim().lines().collect();

    println!("Part 1: TODO");
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let _lines: Vec<&str> = input.trim().lines().collect();

    println!("Part 2: TODO");
    Ok(())
}
