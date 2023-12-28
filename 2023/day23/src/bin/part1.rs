use std::collections::{VecDeque, HashSet};

#[allow(dead_code)]
const INPUT: &'static str = "\
#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

#[allow(dead_code)]
const EXPECTED: &'static str = "94";

fn can_move(c: char, dx: i32, dy: i32) -> bool {
    match (c, dx, dy) {
        ('.', _, _) => true,
        ('>', 0, 1) => true,
        ('<', 0, -1) => true,
        ('^', -1, 0) => true,
        ('v', 1, 0) => true,
        _ => false,
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let sol = solution(input);
    dbg!(sol);
}

fn solution(s: &str) -> String {
    let grid: Vec<Vec<_>> = s.lines().map(|line| line.chars().collect()).collect();
    let nrows = grid.len() as i32;
    let ncols = grid[0].len() as i32;
    let start = (0..ncols).filter(|&i| grid[0][i as usize] == '.').next().expect("no starting point");

    // up, right, down, left
    let neighbours = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut q: VecDeque<(i32, i32, i32, HashSet<(i32, i32)>)> = VecDeque::new();
    // steps, x, y, visited
    q.push_back((0, 0, start, HashSet::new()));

    let mut ans = 0;
    let mut best_visited: HashSet<(i32, i32)> = HashSet::new();
    while let Some((steps, x, y, mut visited)) = q.pop_front() {
        if x == nrows - 1 {
            if steps > ans {
                ans = steps;
                best_visited = visited;
            }
            continue;
        }

        if visited.contains(&(x, y)) {
            continue;
        }
        visited.insert((x, y));

        for (ndx, ndy) in &neighbours {
            let nx  = x + ndx;
            let ny = y + ndy;
            if nx >= 0 && ny >= 0 && nx < nrows && ny < nrows && can_move(grid[nx as usize][ny as usize], *ndx, *ndy) && !visited.contains(&(nx, ny)) {
                q.push_back((steps + 1, nx, ny, visited.clone()));
            }
        }
    }

    let mut ss = String::new();
    for row in 0..nrows {
        for col in 0..ncols {
            if best_visited.contains(&(row, col)) {
                ss.push('O');
            } else {
                ss.push(grid[row as usize][col as usize]);
            }
        }
        ss.push('\n');
    }
    println!("{ss}");

    println!("{ans}");
    return ans.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert!(solution(INPUT) == EXPECTED.to_string());
    }
}
