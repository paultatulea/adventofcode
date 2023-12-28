use std::cmp::{max, min};

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
const EXPECTED: &'static str = "952408144115";

fn main() {
    let input = include_str!("../input.txt");
    let sol = solution(input);
    dbg!(sol);
}

fn op_to_delta(c: char) -> (i128, i128) {
    match c {
        '3' => (-1, 0),
        '0' => (0, 1),
        '1' => (1, 0),
        '2' => (0, -1),
        _ => panic!(),
    }
}

fn solution(s: &str) -> String {
    let ops: Vec<String> = s.lines().map(|line| line.split_whitespace().skip(2).next().unwrap().chars().filter(|c| c.is_digit(16)).collect()).collect();

    let mut nodes: Vec<(i128, i128)> = Vec::new();
    let (mut x, mut y) = (0, 0);
    let mut edge_len = 0i128;
    nodes.push((x, y));
    for (_i, op) in ops.iter().enumerate() {
        let last = op.chars().last().unwrap();
        let (dx, dy) = op_to_delta(last);
        let steps = i128::from_str_radix(&op[0..op.len()-1], 16).unwrap();
        x += dx * steps;
        y += dy * steps;
        edge_len += steps;
        nodes.push((x, y));
    }

    let mut ans = 0i128;
    // trapezoid shoelace formula
    // A = 1/2 * sum((y_i + y_{i+1}) * (x_i - x_{i+1}))
    // Notation flipped in code because rows are represented with x and columns with y.
    for i in 1..nodes.len() {
        let (x1, y1) = nodes[i-1];
        let (x2, y2) = nodes[i];
        let area = (x1 + x2) * (y1 - y2);
        ans += area;
    }
    // trapezoid shoelace formula excludes half of the boundary
    // Pick's formula A = i + b/2 - 1
    // b = boundary, i = interior
    ans = (ans / 2).abs();
    ans += edge_len / 2;
    ans += 1;
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
