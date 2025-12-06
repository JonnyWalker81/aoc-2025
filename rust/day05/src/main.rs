use std::collections::HashSet;
use std::io::{self, Read};
use std::ops::Range;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

// Input:
// 3-5
// 10-14
// 16-20
// 12-18

// 1
// 5
// 8
// 11
// 17
// 32
fn part1(input: &str) -> Result<()> {
    let (id_ranges, ing) = input.split_once("\n\n").unwrap();

    let mut ids: Vec<Range<usize>> = Vec::new();
    for i in id_ranges.lines() {
        let parts: Vec<&str> = i.split('-').collect();
        let start = parts[0].parse::<usize>().unwrap();
        let end = parts[1].parse::<usize>().unwrap();
        let r = start..end;
        ids.push(r);
    }

    let mut ingredients = Vec::new();
    for i in ing.lines() {
        let ingredient: usize = i.parse().unwrap();
        ingredients.push(ingredient);
    }

    let mut sum = 0;
    for i in &ingredients {
        for r in &ids {
            if *i >= r.start && *i <= r.end {
                sum += 1;
                break;
            }
        }
    }

    println!("Part 1: {sum}");
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let (id_ranges, ing) = input.split_once("\n\n").unwrap();

    let mut ids: Vec<Range<usize>> = Vec::new();
    for i in id_ranges.lines() {
        let parts: Vec<&str> = i.split('-').collect();
        let start = parts[0].parse::<usize>().unwrap();
        let end = parts[1].parse::<usize>().unwrap();
        let r = start..end;
        ids.push(r);
    }

    ids.sort_by(|a, b| a.start.cmp(&b.start).then_with(|| a.end.cmp(&b.end)));

    let mut merged = Vec::new();
    let mut cur: Range<usize> = ids.first().unwrap().clone();

    for r in ids.iter().skip(1) {
        if r.start <= cur.end + 1 {
            cur.end = cur.end.max(r.end);
        } else {
            merged.push(cur);
            cur = r.clone();
        }
    }

    merged.push(cur);

    let mut sum = 0;
    for r in merged {
        let s = r.end - r.start + 1;
        sum += s;
    }

    println!("Part 2: {sum}");
    Ok(())
}
