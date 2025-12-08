use std::{
    collections::{HashMap, HashSet},
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
    z: usize,
}

impl Point {
    fn distance(&self, other: &Point) -> usize {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).isqrt() as usize
    }
}

fn part1(input: &str) -> Result<()> {
    let lines: Vec<&str> = input.trim().lines().collect();

    let mut points: Vec<Point> = Vec::new();
    for l in lines {
        let parts: Vec<&str> = l.split(',').collect();
        let x: usize = parts[0].parse().unwrap();
        let y: usize = parts[1].parse().unwrap();
        let z: usize = parts[2].parse().unwrap();
        let p = Point { x, y, z };
        points.push(p);
    }

    let mut idToPoint: HashMap<usize, Point> = HashMap::new();
    let mut pointToId: HashMap<Point, usize> = HashMap::new();
    for (i, p) in points.iter().enumerate() {
        idToPoint.insert(i, *p);
        pointToId.insert(*p, i);
    }

    let distances = sort_distances(&points);
    let mut boxes: Vec<HashSet<usize>> = Vec::new();

    for ((p1, p2), d) in distances.iter().take(1000) {
        let id1 = *pointToId.get(p1).unwrap();
        let id2 = *pointToId.get(p2).unwrap();

        // Find indices of circuits containing id1/id2
        let mut idx1: Option<usize> = None;
        let mut idx2: Option<usize> = None;

        for (i, b) in boxes.iter().enumerate() {
            if idx1.is_none() && b.contains(&id1) {
                idx1 = Some(i);
            }
            if idx2.is_none() && b.contains(&id2) {
                idx2 = Some(i);
            }
            if idx1.is_some() && idx2.is_some() {
                break;
            }
        }

        match (idx1, idx2) {
            (None, None) => {
                boxes.push(HashSet::from([id1, id2]));
            }
            (Some(i), None) => {
                boxes[i].insert(id2);
            }
            (None, Some(j)) => {
                boxes[j].insert(id1);
            }
            (Some(i), Some(j)) => {
                if i != j {
                    // Merge two different circuits
                    let (keep, remove) = if i < j { (i, j) } else { (j, i) };
                    let other = boxes.remove(remove);
                    boxes[keep].extend(other);
                }
                // else: same circuit already, do nothing
            }
        }
    }

    boxes.sort_by_key(|b| b.len());

    let mut sum = 1;
    for b in boxes.iter().rev().take(3) {
        sum *= b.len();
    }

    println!("Part 1: {sum}");
    Ok(())
}

fn sort_distances(points: &Vec<Point>) -> Vec<((Point, Point), usize)> {
    let mut distances: Vec<((Point, Point), usize)> = Vec::new();
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let distance = points[i].distance(&points[j]);
            distances.push(((points[i], points[j]), distance));
        }
    }

    distances.sort_by_key(|(_, d)| *d);
    distances
}

fn find_box_index(boxes: &[HashSet<usize>], id: usize) -> Option<usize> {
    boxes.iter().position(|b| b.contains(&id))
}

fn part2(input: &str) -> Result<()> {
    let lines: Vec<&str> = input.trim().lines().collect();

    let mut points: Vec<Point> = Vec::new();
    for l in lines {
        let parts: Vec<&str> = l.split(',').collect();
        let x: usize = parts[0].parse().unwrap();
        let y: usize = parts[1].parse().unwrap();
        let z: usize = parts[2].parse().unwrap();
        let p = Point { x, y, z };
        points.push(p);
    }

    let mut id_to_point: HashMap<usize, Point> = HashMap::new();
    let mut point_to_id: HashMap<Point, usize> = HashMap::new();
    for (i, p) in points.iter().enumerate() {
        id_to_point.insert(i, *p);
        point_to_id.insert(*p, i);
    }

    let distances = sort_distances(&points);

    // circuits with >= 2 members
    let mut boxes: Vec<HashSet<usize>> = Vec::new();
    // still-alone boxes
    let mut singles: HashSet<usize> = (0..points.len()).collect();

    let mut last_merge: Option<(usize, usize)> = None;

    for ((p1, p2), _d2) in distances.iter() {
        let id1 = *point_to_id.get(p1).unwrap();
        let id2 = *point_to_id.get(p2).unwrap();

        let idx1 = find_box_index(&boxes, id1);
        let idx2 = find_box_index(&boxes, id2);

        match (idx1, idx2) {
            (None, None) => {
                // Both are singles -> new 2-box circuit
                let mut s = HashSet::new();
                s.insert(id1);
                s.insert(id2);
                boxes.push(s);
                singles.remove(&id1);
                singles.remove(&id2);

                last_merge = Some((id1, id2));
            }
            (Some(i), None) => {
                // id1 in a circuit, id2 alone -> add id2
                boxes[i].insert(id2);
                singles.remove(&id2);

                last_merge = Some((id1, id2));
            }
            (None, Some(j)) => {
                // id2 in a circuit, id1 alone -> add id1
                boxes[j].insert(id1);
                singles.remove(&id1);

                last_merge = Some((id1, id2));
            }
            (Some(i), Some(j)) => {
                if i != j {
                    // Merge two different circuits
                    let (keep, remove) = if i < j { (i, j) } else { (j, i) };
                    let other = boxes.remove(remove);
                    boxes[keep].extend(other);

                    last_merge = Some((id1, id2));
                } else {
                    // Same circuit already -> no change
                }
            }
        }

        // Stop when everything is one circuit
        if boxes.len() + singles.len() == 1 {
            break;
        }
    }

    let (a, b) = last_merge.expect("Should have merged at least once");
    let ans = points[a].x * points[b].x;

    println!("Part 2: {ans}");
    Ok(())
}
