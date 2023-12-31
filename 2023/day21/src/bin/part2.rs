use std::collections::HashSet;
use std::collections::VecDeque;

#[allow(dead_code)]
const INPUT: &'static str = "\
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

#[allow(dead_code)]
const EXPECTED: &'static str = "16";

fn main() {
    let input = include_str!("../input.txt");
    let sol = solution(input);
    dbg!(sol);
}

fn visualize(grid: &Vec<Vec<char>>, startrow: i128, startcol: i128, total_steps: i128) {
    let nrows = grid.len() as i128;
    let ncols = grid[0].len() as i128;
    let mut heap = VecDeque::new();
    let mut visited = HashSet::new();
    let mut accepted = HashSet::new();
    heap.push_back((total_steps, startrow, startcol));

    let directions = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    while let Some((steps, row, col)) = heap.pop_front() {
        if steps % 2 == 0 {
            accepted.insert((row, col));
        }
        if steps == 0 {
            continue;
        }
        for (dr, dc) in directions {
            let nr = row + dr;
            let nc = col + dc;
            let mut nr_idx = nr % nrows;
            nr_idx += if nr_idx < 0 { nrows } else { 0 };
            let mut nc_idx = nc % ncols;
            nc_idx += if nc_idx < 0 { ncols } else { 0 };
            if grid[nr_idx as usize][nc_idx as usize] == '#' {
                continue;
            }
            if visited.contains(&(nr, nc)) {
                continue;
            }
            visited.insert((nr, nc));
            heap.push_back((steps - 1, nr, nc));
        }
    }

    let minrow = visited.iter().map(|(row, _)| row).min().unwrap().clone();
    let maxrow = visited.iter().map(|(row, _)| row).max().unwrap().clone();
    let mincol = visited.iter().map(|(_, col)| col).min().unwrap().clone();
    let maxcol = visited.iter().map(|(_, col)| col).max().unwrap().clone();

    let mut ss = String::new();
    let hch = '-';
    let vch = '|';
    for i in minrow..=maxrow {
        let mut nr_idx = i % nrows;
        nr_idx += if nr_idx < 0 { nrows } else { 0 };
        for j in mincol..=maxcol {
            let mut nc_idx = j % ncols;
            nc_idx += if nc_idx < 0 { ncols } else { 0 };

            if nc_idx == 0 {
                ss.push(vch);
            }
            if grid[nr_idx as usize][nc_idx as usize] == '#' {
                ss.push('#');
            } else if accepted.contains(&(i, j)) {
                ss.push('O');
            } else {
                ss.push('.');
            }
        }
        ss.push('\n');
        if nr_idx == nrows - 1 {
            for z in mincol..maxcol + 1 {
                ss.push(hch);
                let mut zz = z % ncols;
                zz += if zz < 0 { ncols } else { 0 };
                if zz == 0 {
                    ss.push(hch);
                }
            }
            ss.push('\n');
        }
    }
    println!("------------");
    println!("{ss}");
}

fn count_grid(
    grid: &Vec<Vec<char>>,
    startrow: i128,
    startcol: i128,
    steps_remaining: i128,
) -> i128 {
    let nrows = grid.len() as i128;
    let ncols = grid[0].len() as i128;
    let mut q = VecDeque::new();
    let mut visited = HashSet::new();
    // Starting point
    q.push_back((steps_remaining, startrow, startcol));
    visited.insert((startrow, startcol));

    let directions = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut ans = 0;
    while let Some((steps, row, col)) = q.pop_front() {
        if steps % 2 == 0 {
            ans += 1;
        }
        if steps == 0 {
            continue;
        }
        for (dr, dc) in directions {
            let nr = row + dr;
            let nc = col + dc;
            if nr < 0 || nc < 0 || nr >= nrows || nc >= ncols {
                continue;
            }
            if grid[nr as usize][nc as usize] == '#' {
                continue;
            }
            if visited.contains(&(nr, nc)) {
                continue;
            }
            visited.insert((nr, nc));
            q.push_back((steps - 1, nr, nc));
        }
    }
    return ans;
}

fn solution(s: &str) -> String {
    let mut ans: i128 = 0;
    let (mut startrow, mut startcol) = (0, 0);
    let grid: Vec<Vec<_>> = s
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .inspect(|(col, c)| {
                    if *c == 'S' {
                        startrow = row as i128;
                        startcol = *col as i128;
                    }
                })
                .map(|(_, c)| c)
                .collect::<Vec<char>>()
        })
        .collect();

    let nrows = grid.len() as i128;
    let ncols = grid[0].len() as i128;

    let total_size = nrows * ncols;
    let big_odd = total_size + if total_size % 2 == 0 { 1 } else { 0 };
    let big_even = total_size + if total_size % 2 == 0 { 0 } else { 1 };

    let odd_steps = count_grid(&grid, startrow, startcol, big_odd);
    let even_steps = count_grid(&grid, startrow, startcol, big_even);

    // Very important that the starting row and column is empty.
    // Also the edges of the grid are empty.
    assert!(nrows % 2 == 1);
    assert!(nrows == ncols);
    // rename since nrows matches ncols
    let N = nrows;

    let steps = 26_501_365;
    // let steps = 65 * 7;
    // visualize(&grid, startrow, startcol, steps);

    let width = (steps / N) - 1;

    // TODO(paul): Math proof
    let n_odd_grids = ((width / 2) * 2 + 1).pow(2);
    let n_even_grids = (((width + 1) / 2) * 2).pow(2);

    println!("{width} {n_odd_grids} {n_even_grids}");

    // Steps remaining at start of last grid in line with start is size of grid - 1
    let steps_at_edge_start = N - 1;
    let left = count_grid(&grid, startrow, N - 1, steps_at_edge_start);
    let right = count_grid(&grid, startrow, 0, steps_at_edge_start);
    let top = count_grid(&grid, N - 1, startcol, steps_at_edge_start);
    let bottom = count_grid(&grid, 0, startcol, steps_at_edge_start);

    // Steps remaining at a corner is half grid width - 1 (e.g. shift over from last grid)
    let steps_at_corner_start = (N - 1) - ((N / 2) + 1);
    let top_right = count_grid(&grid, N - 1, 0, steps_at_corner_start);
    let top_left = count_grid(&grid, N - 1, N - 1, steps_at_corner_start);
    let bottom_right = count_grid(&grid, 0, 0, steps_at_corner_start);
    let bottom_left = count_grid(&grid, 0, N - 1, steps_at_corner_start);

    // Steps remaining at wedge is half grid shifted from two grids away from end.
    // (2N - 1) - [(N // 2) + 1]
    let steps_at_wedge_start = (2 * N - 1) - ((N / 2) + 1);
    let top_right_wedge = count_grid(&grid, N - 1, 0, steps_at_wedge_start);
    let top_left_wedge = count_grid(&grid, N - 1, N - 1, steps_at_wedge_start);
    let bottom_right_wedge = count_grid(&grid, 0, 0, steps_at_wedge_start);
    let bottom_left_wedge = count_grid(&grid, 0, N - 1, steps_at_wedge_start);

    ans = (n_odd_grids * odd_steps + n_even_grids * even_steps)
        + (left + right + top + bottom)
        + (width + 1) * (top_right + top_left + bottom_right + bottom_left)
        + width * (top_right_wedge + top_left_wedge + bottom_right_wedge + bottom_left_wedge);

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
