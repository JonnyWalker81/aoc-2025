use std::collections::HashSet;
use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<dyn::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let lines: Vec<&str> = input.trim().lines().collect();

    let mut sum = 0;
    for l in lines {
        let mut digits: Vec<u8> = Vec::new();
        for c in l.chars() {
            let d = c.to_digit(10).unwrap();
            digits.push(d as u8);
        }

        let mut max_left_digit: i32 = -1;
        let mut best_value: i32 = 0;
        for i in 0..digits.len() {
            let d = digits[i] as i32;
            if max_left_digit > 0 {
                let candidate = 10 * max_left_digit + d;
                best_value = best_value.max(candidate);
            }
            max_left_digit = max_left_digit.max(d);
        }

        println!("{best_value}");
        sum += best_value;
    }

    println!("Part 1: {sum}");
    Ok(())
}

fn max_subsequence_stack(s: Vec<u32>, k: u32) -> Vec<u32> {
    let n: u32 = s.len() as u32;
    let mut to_remove = n - k;
    let mut stack = Vec::new();

    for d in s {
        while !stack.is_empty() && to_remove > 0 && *stack.last().unwrap() < d {
            stack.pop();
            to_remove -= 1;
        }

        stack.push(d);
    }

    while to_remove > 0 {
        stack.pop();
        to_remove -= 1;
    }

    let result = stack.iter().cloned().take(k as usize).collect();

    result
}

fn digits_to_u64(digits: &[u32]) -> u64 {
    digits.iter().fold(0u64, |acc, &d| acc * 10 + d as u64)
}

fn part2(input: &str) -> Result<()> {
    let lines: Vec<&str> = input.trim().lines().collect();

    let mut sum = 0;
    for l in lines {
        let mut digits: Vec<u32> = Vec::new();
        for c in l.chars() {
            let d = c.to_digit(10).unwrap();
            digits.push(d);
        }

        let ms = max_subsequence_stack(digits.clone(), 12);
        let num = digits_to_u64(&ms);
        println!("{num}");
        sum += num;
    }

    println!("Part 2: {sum}");
    Ok(())
}
