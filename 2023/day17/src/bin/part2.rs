use std::collections::BinaryHeap;
use std::collections::HashSet;

#[allow(dead_code)]
const INPUT: &'static str = "\
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

#[allow(dead_code)]
const EXPECTED: &'static str = "94";

fn main() {
    let input = include_str!("../input.txt");
    let sol = solution(input);
    dbg!(sol);
}

fn solution(s: &str) -> String {
    let grid: Vec<Vec<u32>> = s.lines().map(|line| line.chars().map(|x| x.to_digit(10).unwrap()).collect()).collect();
    let nrows = grid.len() as i32;
    let ncols = grid[0].len() as i32;
    let neighbours = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    let mut q: BinaryHeap<(i32, i32, i32, i32, i32, u8)> = BinaryHeap::new();
    let mut seen = HashSet::new();
    // cost, x, y, dx, dy, consecutive
    q.push((0, 0, 0, 0, 0, 0));

    let mut ans = 0;
    while !q.is_empty() {
        let curr = q.pop().unwrap();
        let (cost, x, y, dx, dy, consecutive) = curr;

        if x == nrows - 1 && y == ncols - 1 {
            if consecutive >= 4 {
                ans = -cost;
                break;
            } else {
                continue;
            }
        }

        if seen.contains(&(x, y, dx, dy, consecutive)) {
            continue;
        }
        seen.insert((x, y, dx, dy, consecutive));

        for &(ndx, ndy) in &neighbours {
            // Cannot go in reverse
            if ndx == -dx && ndy == -dy {
                continue;
            }
            if (dx, dy) != (0, 0) && consecutive < 4 && (dx, dy) != (ndx, ndy) {
                continue;
            }
            let new_consecutive = if ndx == dx && ndy == dy { consecutive + 1 } else { 1 };
            if new_consecutive > 10 && (dx, dy) == (ndx, ndy) {
                continue;
            }

            let nx = x + ndx;
            let ny = y + ndy;
            // Check bounds of grid
            if nx < 0 || nx >= nrows || ny < 0 || ny >= ncols {
                continue;
            }
            let new_cost = cost - grid[nx as usize][ny as usize] as i32;
            q.push((new_cost, nx, ny, ndx, ndy, new_consecutive));
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
