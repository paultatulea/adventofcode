use std::cmp::{max, min};
use std::collections::HashSet;
use std::collections::VecDeque;

#[allow(dead_code)]
const INPUT: &'static str = "\
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

#[allow(dead_code)]
const EXPECTED: &'static str = "62";

fn main() {
    let input = include_str!("../input.txt");
    let sol = solution(input);
    dbg!(sol);
}

fn op_to_delta(c: &str) -> (i32, i32) {
    match c {
        "U" => (-1, 0),
        "R" => (0, 1),
        "D" => (1, 0),
        "L" => (0, -1),
        _ => panic!(),
    }
}

fn solution(s: &str) -> String {
    let ops: Vec<Vec<_>> = s.lines().map(|line| line.split_whitespace().take(2).collect()).collect();

    let mut path = HashSet::new();
    let (mut x, mut y) = (0, 0);
    let (mut minx, mut miny, mut maxx, mut maxy) = (0, 0, 0, 0);
    path.insert((x, y));
    for op in ops {
        let (dx, dy) = op_to_delta(op[0]);
        let steps: i32 = op[1].parse().unwrap();
        for _ in 0..steps {
            x += dx;
            y += dy;
            path.insert((x, y));
            minx = min(minx, x);
            miny = min(miny, y);
            maxx = max(maxx, x);
            maxy = max(maxy, y);
        }
    }

    let mut ans = 0;
    let mut ss = String::new();
    let directions: [(i32, i32);4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    for r in minx..=maxx {
        for c in miny..=maxy {
            if path.contains(&(r, c)) {
                ss.push('#');
                ans += 1;
            } else {
                let intersects: Vec<_> = directions.iter().map(|&(dx, dy)| {
                    let perpendicular = [(-dy, -dx), (dy, dx)];
                    let (mut row, mut col) = (r + dx, c + dy);
                    let mut crosses = 0;
                    let mut wall_start: Option<(i32, i32)> = None;
                    while row >= minx && col >= miny && row <= maxx && col <= maxy {
                        if path.contains(&(row, col)) {
                            if wall_start.is_none() {
                                let sides: Vec<_> = perpendicular.iter().filter(|&(dxx, dyy)| path.contains(&(row + dxx, col + dyy))).map(|&(dxx, dyy)| (dxx, dyy)).collect();
                                if sides.len() == 2 {
                                    // Encountered perpendicular wall
                                    crosses += 1;
                                    wall_start = None;
                                } else {
                                    wall_start = sides.iter().next().copied();
                                }
                            }
                        } else if let Some((dxx, dyy)) = wall_start {
                            // Check the previous point for a wall on opposite side as start.
                            let (prev_x, prev_y) = (row - dx, col - dy);
                            if path.contains(&(prev_x - dxx, prev_y - dyy)) {
                                crosses += 1;
                            }
                            wall_start = None;
                        }
                        row += dx;
                        col += dy;
                    }
                    // Final in case the wall goes to the bounds of the grid.
                    if let Some((dxx, dyy)) = wall_start {
                        // Check the previous point for a wall on opposite side as start.
                        let (prev_x, prev_y) = (row - dx, col - dy);
                        if path.contains(&(prev_x - dxx, prev_y - dyy)) {
                            crosses += 1;
                        }
                    }
                    return crosses;
                }).collect();
                if intersects.iter().all(|&x| x % 2 != 0 && x > 0) {
                    ans += 1;
                    ss.push('#');
                } else {
                    ss.push('.');
                }
            }

        }
        ss.push('\n');
    }
    println!("{ss}");
    println!("{ans}");
    ans.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert!(solution(INPUT) == EXPECTED.to_string());
    }
}
