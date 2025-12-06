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
    let lines: Vec<&str> = input.trim().lines().collect();

    let mut rows: Vec<Vec<&str>> = Vec::new();
    for line in lines {
        let parts: Vec<_> = line.split(' ').filter(|x| x.len() > 0).collect();
        println!("{:?}", parts);
        rows.push(parts);
    }

    let mut cols: Vec<Vec<&str>> = Vec::new();
    for c in 0..rows[0].len() {
        let mut col: Vec<&str> = Vec::new();
        for r in 0..rows.len() {
            col.push(rows[r][c]);
        }
        cols.push(col);
    }

    for c in 0..cols.len() {
        for row in 0..rows.len() {
            print!("{} ", cols[c][row]);
        }
        println!();
    }

    let mut sums = Vec::new();
    for c in 0..cols.len() {
        let mut prod = 1;
        let mut sum = 0;
        let last = cols[c].last().unwrap();
        for row in 0..rows.len() - 1 {
            let n = cols[c][row].parse::<usize>().unwrap();
            if *last == "*" {
                println!("mult: {n}");
                prod *= n;
            } else if *last == "+" {
                sum += n;
            }
        }
        if *last == "*" {
            sums.push(prod);
        } else if *last == "+" {
            sums.push(sum);
        }
    }

    for s in &sums {
        println!("{}", s);
    }

    let total = sums.iter().sum::<usize>();

    println!("Part 1: {total}");
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let lines: Vec<&str> = input.trim().lines().collect();

    let max_w = lines.iter().map(|l| l.len()).max().unwrap_or(0);

    let mut rows: Vec<Vec<char>> = Vec::new();
    for line in lines {
        let mut parts: Vec<char> = line.chars().collect();
        parts.resize(max_w, ' ');
        // println!("{:?}", parts); // optional debug
        rows.push(parts);
    }

    let mut cols: Vec<Vec<char>> = Vec::new();
    for c in 0..rows[0].len() {
        let mut col: Vec<char> = Vec::new();
        for r in 0..rows.len() {
            col.push(rows[r][c]);
        }
        cols.push(col);
    }

    let mut blocks: Vec<Vec<Vec<char>>> = Vec::new();
    let mut cur: Vec<Vec<char>> = Vec::new();
    for col in cols {
        let is_sep = col.iter().all(|ch| *ch == ' ');
        if is_sep {
            if !cur.is_empty() {
                blocks.push(cur);
                cur = Vec::new();
            }
        } else {
            cur.push(col);
        }
    }
    if !cur.is_empty() {
        blocks.push(cur);
    }

    let mut total: usize = 0;

    for r in 0..blocks.len() {
        let block = &blocks[r];

        // Operator row is the last row in the grid.
        // Find the operator somewhere in the bottom row across this block.
        let mut op: Option<char> = None;
        for col in block.iter() {
            if let Some(ch) = col.last() {
                if *ch == '*' || *ch == '+' {
                    op = Some(*ch);
                    break;
                }
            }
        }
        let op = op.unwrap();

        // Read numbers by columns RIGHT-to-LEFT.
        // Each column (excluding bottom operator row) forms one number top->bottom.
        let mut nums: Vec<usize> = Vec::new();
        for col in block.iter().rev() {
            let mut cur = String::new();
            for i in 0..col.len().saturating_sub(1) {
                let ch = col[i];
                if ch.is_ascii_digit() {
                    cur.push(ch);
                }
            }
            if !cur.is_empty() {
                let val = cur.parse::<usize>().unwrap();
                nums.push(val);
            }
        }

        let mut sum: usize = 0;
        let mut prod: usize = 1;
        for i in 0..nums.len() {
            if op == '*' {
                prod *= nums[i];
            } else if op == '+' {
                sum += nums[i];
            }
        }

        if op == '*' {
            total += prod;
            // println!("{prod}"); // optional debug
        } else if op == '+' {
            total += sum;
            // println!("{sum}"); // optional debug
        }
    }

    println!("Part 2: {total}");
    Ok(())
}
