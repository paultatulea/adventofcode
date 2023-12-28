#[allow(dead_code)]
const INPUT: &'static str = "\
..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

#[allow(dead_code)]
const EXPECTED: &'static str = "8";

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

    let mut stack: Vec<(isize, isize, (isize, isize), usize)> = Vec::new();
    for (dx, dy) in directions {
        let (x, y) = (startx + dx, starty + dy);
        if should_visit(x, y, maxx, maxy, &grid) {
            stack.push((x, y, (-dx, -dy), 1));
        }
    }

    let mut ans: usize = 0;
    while !stack.is_empty() {
        let (thisx, thisy, from, steps) = stack.pop().unwrap();
        let tile = grid[thisx as usize][thisy as usize];
        if tile == 'S' {
            ans = (steps + 1) / 2;
            break;
        }
        if let Some((dx, dy)) = next_tile(tile, from) {
            let (x, y) = (thisx + dx, thisy + dy);
            if should_visit(x, y, maxx, maxy, &grid) {
                stack.push((x, y, (-dx, -dy), steps + 1));
            }
        }
    }

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
