use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::collections::HashSet;
use std::{
    cmp::Reverse,
    collections::HashMap,
    io::{self, Read},
};

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

    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut start_row = 0;
    let mut start_col = 0;
    let mut row = 0;
    for line in lines {
        let parts: Vec<char> = line.chars().collect();
        // println!("{:?}", parts); // optional debug
        for i in 0..parts.len() {
            if parts[i] == 'S' {
                start_row = row;
                start_col = i;
            }
        }
        row += 1;
        grid.push(parts);
    }

    let mut tachyons: HashSet<(usize, usize)> = HashSet::new(); // vec![(start_row + 1, start_col)];
                                                                // let mut tachyons = vec![(start_row + 1, start_col)];
    tachyons.insert((start_row + 1, start_col));
    let mut count = 0;
    loop {
        let mut did_move = false;
        let copy = tachyons.clone();
        for t in copy {
            let next = (t.0 + 1, t.1);
            if next.0 >= grid.len() || next.1 >= grid[next.0].len() {
                continue;
            } else {
                let cell = grid[next.0][next.1];
                if cell == '^' {
                    // split tachyon
                    let t1 = (next.0, next.1 - 1);
                    let t2 = (next.0, next.1 + 1);
                    // tachyons.push(t1);
                    // tachyons.push(t2);
                    // tachyons.remove(i);

                    tachyons.insert(t1);
                    tachyons.insert(t2);
                    tachyons.remove(&t);
                    count += 1;
                } else {
                    tachyons.remove(&t);
                    tachyons.insert(next);
                }
                did_move = true;
            }
        }
        if !did_move {
            break;
        }
    }

    println!("Part 1: {count}");
    Ok(())
}

fn _print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
}

fn part2(input: &str) -> Result<()> {
    let lines: Vec<&str> = input.trim().lines().collect();
    let rows = lines.len();
    let cols = lines.get(0).map(|l| l.len()).unwrap_or(0);

    let mut grid: Vec<Vec<u8>> = Vec::with_capacity(rows);
    let mut s_pos = (0usize, 0usize);
    let mut splitters: Vec<(usize, usize)> = Vec::new();

    for (r, line) in lines.iter().enumerate() {
        let bytes = line.as_bytes().to_vec();
        for (c, &ch) in bytes.iter().enumerate() {
            if ch == b'S' {
                s_pos = (r, c);
            } else if ch == b'^' {
                splitters.push((r, c));
            }
        }
        grid.push(bytes);
    }

    let n = splitters.len();

    // per-column splitter rows
    let mut col_rows: Vec<Vec<usize>> = vec![Vec::new(); cols];
    for &(r, c) in &splitters {
        col_rows[c].push(r);
    }
    for v in &mut col_rows {
        v.sort_unstable();
    }

    // id map
    let mut id_of: HashMap<(usize, usize), usize> = HashMap::with_capacity(n * 2 + 1);
    for (i, &(r, c)) in splitters.iter().enumerate() {
        id_of.insert((r, c), i);
    }

    // children
    let mut left: Vec<Option<usize>> = vec![None; n];
    let mut right: Vec<Option<usize>> = vec![None; n];

    for (i, &(r, c)) in splitters.iter().enumerate() {
        let c_i = c as isize;

        if let Some(lr) = next_row(&col_rows, c_i - 1, r) {
            left[i] = id_of.get(&(lr, (c_i - 1) as usize)).copied();
        }
        if let Some(rr) = next_row(&col_rows, c_i + 1, r) {
            right[i] = id_of.get(&(rr, (c_i + 1) as usize)).copied();
        }
    }

    // order bottom-up
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by_key(|&i| Reverse(splitters[i].0));

    // DP
    let mut f: Vec<BigUint> = vec![BigUint::zero(); n];
    for i in order {
        let lc = match left[i] {
            None => BigUint::one(),
            Some(j) => f[j].clone(),
        };
        let rc = match right[i] {
            None => BigUint::one(),
            Some(j) => f[j].clone(),
        };
        f[i] = lc + rc;
    }

    // start: first splitter below S in S column
    let (sr, sc) = s_pos;
    let first = next_row(&col_rows, sc as isize, sr + 1);

    let ans = match first {
        None => BigUint::one(),
        Some(fr) => {
            let id = id_of[&(fr, sc)];
            f[id].clone()
        }
    };

    println!("Part 2: {ans}");
    Ok(())
}

fn lower_bound(a: &[usize], x: usize) -> usize {
    let mut lo = 0usize;
    let mut hi = a.len();
    while lo < hi {
        let mid = (lo + hi) / 2;
        if a[mid] < x {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    lo
}

fn next_row(cols_rows: &Vec<Vec<usize>>, c: isize, r: usize) -> Option<usize> {
    if c < 0 {
        return None;
    }
    let c = c as usize;
    if c >= cols_rows.len() {
        return None;
    }
    let v = &cols_rows[c];
    let i = lower_bound(v, r);
    if i == v.len() {
        None
    } else {
        Some(v[i])
    }
}
