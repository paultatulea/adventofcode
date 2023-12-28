use std::collections::{VecDeque, HashSet, HashMap};
use std::time::{Duration, Instant};

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
const EXPECTED: &'static str = "154";

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

    let mut graph: HashMap<(i32, i32), HashSet<(i32, i32, i32)>> = HashMap::new();
    let neighbours = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    {
        // up, right, down, left
        let mut q: VecDeque<(i32, i32, i32, i32, i32, i32)> = VecDeque::new();
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        // steps, x, y, prev_steps, prev_x, prev_y
        q.push_back((0, 0, start, 0, 0, start));

        while let Some((steps, x, y, prev_steps, prev_x, prev_y)) = q.pop_back() {
            // Check if goal
            if x == nrows - 1 {
                let dist = steps - prev_steps;
                graph.entry((x, y)).and_modify(|e| { e.insert((prev_x, prev_y, dist)); }).or_insert(HashSet::from_iter(vec![(prev_x, prev_y, dist)]));
                graph.entry((prev_x, prev_y)).and_modify(|e| { e.insert((x, y, dist)); }).or_insert(HashSet::from_iter(vec![(x, y, dist)]));
                continue;
            }
            // Always check if this is an intersection node.
            let mut branches = Vec::new();
            for (ndx, ndy) in &neighbours {
                let nx  = x + ndx;
                let ny = y + ndy;
                if nx >= 0 && ny >= 0 && nx < nrows && ny < nrows && grid[nx as usize][ny as usize] != '#' {
                    branches.push((nx, ny));
                }
            }
            let mut new_node_steps = prev_steps;
            let mut new_node_x = prev_x;
            let mut new_node_y = prev_y;
            if branches.len() > 2 && (x, y) != (prev_x, prev_y) {
                let dist = steps - prev_steps;
                graph.entry((x, y)).and_modify(|e| { e.insert((prev_x, prev_y, dist)); }).or_insert(HashSet::from_iter(vec![(prev_x, prev_y, dist)]));
                graph.entry((prev_x, prev_y)).and_modify(|e| { e.insert((x, y, dist)); }).or_insert(HashSet::from_iter(vec![(x, y, dist)]));
                new_node_steps = steps;
                new_node_x = x;
                new_node_y = y;
            }
            if visited.contains(&(x, y)) {
                continue;
            }
            visited.insert((x, y));
        
            for (nx, ny) in &branches {
                q.push_back((steps + 1, *nx, *ny, new_node_steps, new_node_x, new_node_y));
            }
        }
    }

    // for (node, edges) in &graph {
    //     println!("{node:?} -> {edges:?}");
    // }

    let mut q: VecDeque<(i32, i32, i32, HashSet<(i32, i32)>)> = VecDeque::new();
    q.push_back((0, 0, start, HashSet::new()));

    let start = Instant::now();
    let mut i = 0;
    let mut longest = 0;
    while let Some((steps, x, y, mut visited)) = q.pop_front() {
        i += 1;
        if visited.contains(&(x, y)) {
            continue;
        }
        if x == nrows - 1 {
            if steps > longest {
                longest = steps;
            }
            continue;
        }
        visited.insert((x, y));
        for (ex, ey, dist) in graph.get(&(x, y)).expect("No edges") {
            q.push_back((steps + dist, *ex, *ey, visited.clone()));
        }
    }
    let duration = start.elapsed();
    println!("{i} iterations, {duration:?} elapsed");

    let ans = longest;

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
