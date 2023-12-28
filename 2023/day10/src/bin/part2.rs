use std::collections::HashSet;

#[allow(dead_code)]
const INPUT: &'static str = "\
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

#[allow(dead_code)]
const EXPECTED: &'static str = "10";

fn main() {
    let input = include_str!("../input.txt");
    let sol = solution(input);
    dbg!(sol);
}

/*
    | is a vertical pipe connecting north and south.
    - is a horizontal pipe connecting east and west.
    L is a 90-degree bend connecting north and east.
    J is a 90-degree bend connecting north and west.
    7 is a 90-degree bend connecting south and west.
    F is a 90-degree bend connecting south and east.
    . is ground; there is no pipe in this tile.
    S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.
*/

fn next_tile(tile: char, from: (isize, isize)) -> Option<(isize, isize)> {
    match (tile, from) {
        ('|', (-1, 0)) => Some((1, 0)),
        ('|', (1, 0)) => Some((-1, 0)),
        ('-', (0, -1)) => Some((0, 1)),
        ('-', (0, 1)) => Some((0, -1)),
        ('L', (-1, 0)) => Some((0, 1)),
        ('L', (0, 1)) => Some((-1, 0)),
        ('J', (0, -1)) => Some((-1, 0)),
        ('J', (-1, 0)) => Some((0, -1)),
        ('F', (0, 1)) => Some((1, 0)),
        ('F', (1, 0)) => Some((0, 1)),
        ('7', (0, -1)) => Some((1, 0)),
        ('7', (1, 0)) => Some((0, -1)),
        _ => None,
    }
}

fn should_visit(x: isize, y: isize, maxx: isize, maxy: isize, grid: &Vec<Vec<char>>) -> bool {
    return x >= 0 && x < maxx && y >= 0 && y < maxy && grid[x as usize][y as usize] != '.';
}

fn solution(s: &str) -> String {
    let (mut startx, mut starty): (isize, isize) = (0, 0);
    let mut grid: Vec<Vec<char>> = Vec::new();
    for (i, line) in s.lines().enumerate() {
        let mut row: Vec<char> = Vec::new();
        for (j, c) in line.chars().enumerate() {
            row.push(c);
            if c == 'S' {
                (startx, starty) = (i as isize, j as isize);
            }
        }
        grid.push(row);
    }

    let maxx = grid.len() as isize;
    let maxy = grid[0].len() as isize;

    let directions = [(-1, 0), (0, 1), (-1, 0), (0, -1)];

    let mut stack: Vec<(isize, isize, (isize, isize), HashSet<(isize, isize)>)> = Vec::new();
    for (dx, dy) in directions {
        let (x, y) = (startx + dx, starty + dy);
        if should_visit(x, y, maxx, maxy, &grid) {
            let mut set: HashSet<(isize, isize)> = HashSet::new();
            set.insert((startx, starty));
            stack.push((x, y, (-dx, -dy), set));
        }
    }

    let mut path: HashSet<(isize, isize)> = HashSet::default();
    while !stack.is_empty() {
        let (thisx, thisy, from, mut path_set) = stack.pop().unwrap();
        let tile = grid[thisx as usize][thisy as usize];
        if tile == 'S' {
            path = path_set;
            break;
        }
        path_set.insert((thisx, thisy));
        if let Some((dx, dy)) = next_tile(tile, from) {
            let (x, y) = (thisx + dx, thisy + dy);
            if should_visit(x, y, maxx, maxy, &grid) {
                stack.push((x, y, (-dx, -dy), path_set));
            }
        }
    }

    // Determined S is a J through visual inspection but this can be coded up easily enough.
    grid[startx as usize][starty as usize] = 'J';

    let mut inside: HashSet<(isize, isize)> = HashSet::new();

    let mut ans = 0;
    for i in 0..maxx {
        for j in 0..maxy {
            if path.contains(&(i, j)) {
                continue;
            }
            let mut crossed = vec![0; directions.len()];
            for (dir_idx, (dx, dy)) in directions.iter().enumerate() {
                let (mut x, mut y) = (i, j);
                let mut pipe_start: Option<char> = None;
                let bar = if *dy == 0 { '-' } else { '|' };
                while x >= 0 && x < maxx && y >= 0 && y < maxy {
                    let tile = grid[x as usize][y as usize];
                    if path.contains(&(x, y)) {
                        if tile == bar {
                            crossed[dir_idx] += 1;
                            pipe_start = None;
                        } else if tile == 'F' || tile == 'J' || tile == '7' || tile == 'L' {
                            if let Some(start_tile) = pipe_start {
                                if match (tile, start_tile) {
                                    ('F', 'J') | ('7', 'L') | ('J', 'F') | ('L', '7') => true,
                                    _ => false,
                                } {
                                    crossed[dir_idx] += 1;
                                }
                                pipe_start = None;
                            } else {
                                pipe_start = Some(tile);
                            }
                        }
                    } else {
                        pipe_start = None;
                    }
                    x += dx;
                    y += dy;
                }
            }
            if crossed.iter().all(|&c| c > 0 && c % 2 != 0) {
                inside.insert((i, j));
                ans += 1;
            }
        }
    }

    let mut debug: String = String::new();
    for i in 0..maxx {
        for j in 0..maxy {
            if path.contains(&(i, j)) {
                debug.push(' ');
                // debug.push(grid[i as usize][j as usize]);
            } else if inside.contains(&(i, j)) {
                debug.push('#');
            } else {
                debug.push(' ');
            }
        }
        debug.push('\n');
    }

    println!("\n{debug}\n");
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
