use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[allow(dead_code)]
const INPUT: &'static str = "\
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

#[allow(dead_code)]
const EXPECTED: &'static str = "374";


#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let sol = solution(input);
    dbg!(sol);
}


fn get_adjacent(i: usize, n_cols: usize) -> Vec<isize> {
    let maybe_right = i as isize + 1;
    let right: isize = if maybe_right % n_cols as isize != 0 { maybe_right } else { -1 };
    let maybe_left = i as isize - 1;
    let left = if (i % n_cols) != 0 { maybe_left } else { -1 };
    vec![(i as isize - n_cols as isize), right as isize, (i + n_cols) as isize, left]
}


fn djikstra(start: usize, end: usize, n_rows: usize, n_cols: usize, edges: &Vec<Vec<usize>>) -> usize {
    let mut q = BinaryHeap::new();
    let size = n_rows * n_cols;
    let mut dist = vec![usize::MAX; n_rows * n_cols];
    // up, right, down, left
    q.push(State { cost: 0, position: start });
    while let Some(State { cost, position }) = q.pop() {
        // Found the goal
        if position == end {
            return cost;
        }
        // Have reached this node through a shorter path
        if dist[position] < cost {
            continue;
        }
        for neighbor in get_adjacent(position, n_cols) {
            if neighbor < 0 || neighbor >= size as isize {
                continue;
            }
            let next_state = State { cost: cost + edges[position][neighbor as usize], position: neighbor as usize };
            if next_state.cost < dist[neighbor as usize] {
                dist[neighbor as usize] = next_state.cost;
                q.push(next_state);
            }
        }
    }
    unreachable!()
}

fn solution(s: &str) -> String {
    let mut grid: Vec<char> = Vec::new();
    let mut galaxies: Vec<usize> = Vec::new();
    let mut n_cols = 0;

    for (i, line) in s.lines().enumerate() {
        for c in line.chars() {
            if c == '#' {
                galaxies.push(grid.len());
            }
            grid.push(c);
        }
        if i == 0 {
            n_cols = grid.len(); // Store the length of columns on first iteration
        }
    }

    let n_rows = grid.len() / n_cols;

    let mut expand_rows = vec![false; n_rows];
    let mut expand_cols = vec![false; n_cols];

    // Determine the rows to expand
    // Scan rows
    for i in 0..n_rows {
        let offset = i * n_cols;
        if grid[offset..offset+n_cols].iter().all(|c| *c == '.') {
            expand_rows[i] = true;
        }
    }
    // Scan columns
    for j in 0..n_cols {
        if (0..n_rows).map(|row| grid[row * n_cols + j]).all(|c| c == '.') {
            expand_cols[j] = true;
        }
    }

    let mut adj_matrix: Vec<Vec<usize>> = vec![vec![0; grid.len()]; grid.len()];
    let expand = 2;
    for i in 0..grid.len() {
        // up, right, down, left
        let neighbors = get_adjacent(i, n_cols);
        for (n, &neighbor) in neighbors.iter().enumerate() {
            if neighbor < 0 || neighbor >= grid.len() as isize {
                // Not a valid point on the grid.
                continue;
            }
            let row = neighbor / n_cols as isize;
            let col = neighbor % n_cols as isize;
            
            if (n % 2 == 0 && expand_rows[row as usize]) || (n % 2 != 0 && expand_cols[col as usize]) {
                adj_matrix[i][neighbor as usize] = expand;
            } else {
                adj_matrix[i][neighbor as usize] = 1;
            }
        }
    }

    let mut pairs = Vec::new();
    for i in 0..galaxies.len() {
        for j in i+1..galaxies.len() {
            pairs.push((galaxies[i], galaxies[j]));
        }
    }

    let ans = pairs.iter().fold(0, |acc, (start, goal)| {
        let shortest_path = djikstra(*start, *goal, n_rows, n_cols, &adj_matrix);
        acc + shortest_path
    });

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
