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

    let mut cells: Vec<Vec<char>> = Vec::new();

    for l in lines {
        let mut row = Vec::new();
        for c in l.chars() {
            row.push(c);
        }

        cells.push(row);
    }

    // print_table(&cells);

    let to_remove = find_rolls(&mut cells);
    let total = to_remove.len();

    println!("Part 1: {total}");
    Ok(())
}

fn find_rolls(cells: &mut [Vec<char>]) -> Vec<(usize, usize)> {
    let dirs = [
        (0, 1),
        (1, 0),
        (0, -1),
        (-1, 0),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];

    let mut to_remove: Vec<(usize, usize)> = Vec::new();
    for r in 0..cells.len() {
        for c in 0..cells[r].len() {
            if cells[r][c] == '.' {
                continue;
            }

            let mut count = 0;
            for d in &dirs {
                let mut rr = r as isize + d.0;
                let mut cc = c as isize + d.1;
                if rr < 0 || rr >= cells.len() as isize || cc < 0 || cc >= cells[r].len() as isize {
                    continue;
                }

                if cells[rr as usize][cc as usize] == '@' {
                    count += 1;
                }
            }

            if count < 4 {
                to_remove.push((r, c));
            }
        }
    }

    return to_remove;
}

fn print_table(cells: &[Vec<char>]) {
    for r in 0..cells.len() {
        for c in 0..cells[r].len() {
            print!("{}", cells[r][c])
        }
        println!();
    }
}

fn part2(input: &str) -> Result<()> {
    let lines: Vec<&str> = input.trim().lines().collect();

    let mut cells: Vec<Vec<char>> = Vec::new();

    for l in lines {
        let mut row = Vec::new();
        for c in l.chars() {
            row.push(c);
        }

        cells.push(row);
    }

    // print_table(&cells);

    let mut total = 0;
    loop {
        let to_remove = find_rolls(&mut cells);
        let count = to_remove.len();
        if count == 0 {
            break;
        }

        total += count;

        for (r, c) in to_remove {
            cells[r][c] = '.';
        }
    }

    println!("Part 2: {total}");
    Ok(())
}
