use std::cmp;
use std::collections::{HashMap, HashSet, VecDeque};

#[allow(dead_code)]
const INPUT: &'static str = "\
1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

#[allow(dead_code)]
const EXPECTED: &'static str = "7";

fn main() {
    let input = include_str!("../input.txt");
    let sol = solution(input);
    dbg!(sol);
}

fn check_overlap(a: &Vec<Vec<i32>>, b: &Vec<Vec<i32>>) -> bool {
    let a_x_start = a[0][0];
    let a_x_end = a[1][0] + 1;
    let a_y_start = a[0][1];
    let a_y_end = a[1][1] + 1;

    let b_x_start = b[0][0];
    let b_x_end = b[1][0] + 1;
    let b_y_start = b[0][1];
    let b_y_end = b[1][1] + 1;

    let x_overlap_start = cmp::max(a_x_start, b_x_start);
    let x_overlap_end = cmp::min(a_x_end, b_x_end);
    let y_overlap_start = cmp::max(a_y_start, b_y_start);
    let y_overlap_end = cmp::min(a_y_end, b_y_end);

    return x_overlap_start < x_overlap_end && y_overlap_start < y_overlap_end;
}

fn solution(s: &str) -> String {
    let mut blocks: Vec<Vec<Vec<i32>>> = s.lines().map(|line| line.split("~").map(|x| x.split(",").map(|x| x.parse::<i32>().unwrap()).collect()).collect()).collect();
    let mut supports: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut supporting: HashMap<usize, Vec<usize>> = HashMap::new();

    loop {
        let mut shifted_blocks = Vec::new();
        for (i, block) in blocks.iter().enumerate() {
            let minz = block.iter().map(|p| p[2]).min().unwrap();
            if minz == 1 {
                continue;
            }
            if blocks.iter().enumerate().any(|(j, other)| {
                // Skip comparison against self
                if i == j {
                    return false;
                }
                // Cannot move if other block is one unit below and has an x-y overlap
                let other_maxz = other.iter().map(|p| p[2]).max().unwrap();
                minz == other_maxz + 1 && check_overlap(&block, &other)
            }) {
                continue;
            }
            shifted_blocks.push(i);
        }

        for shifted_block in &shifted_blocks {
            for p in blocks[*shifted_block].iter_mut() {
                p[2] -= 1;
            }
        }

        if shifted_blocks.len() == 0 {
            break;
        }
    }

    for (i, block) in blocks.iter().enumerate() {
        let minz = block.iter().map(|p| p[2]).min().unwrap();
        for (j, other) in blocks.iter().enumerate() {
            if i == j {
                continue;
            }
            let other_max = other.iter().map(|p| p[2]).max().unwrap();
            if other_max == minz - 1 && check_overlap(block, other) {
                supports.entry(i).and_modify(|e| e.push(j)).or_insert(vec![j]);
                supporting.entry(j).and_modify(|e| e.push(i)).or_insert(vec![i]);
            }
        }
    }
    let mut ans = 0;
    for (i, _block) in blocks.iter().enumerate() {
        let mut deleted = HashSet::new();
        let mut deque = VecDeque::new();
        deque.push_back(i);
        let mut blocks_fall = -1;

        while let Some(b) = deque.pop_front() {
            deleted.insert(b);
            blocks_fall += 1;
            // Check all the blocks supported by the deleted block
            if let Some(deps) = supporting.get(&b) {
                // For each block that was supported, check if all of it's supports are now
                // deleted.
                for dep in deps {
                    if deleted.contains(&dep) {
                        continue;
                    }
                    let sups = supports.get(&dep).unwrap();
                    if sups.iter().all(|d| deleted.contains(&d)) {
                        deque.push_back(*dep);
                    }
                }
            }
        }
        ans += blocks_fall;
    }
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
