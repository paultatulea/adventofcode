use std::cmp::{max, min};

#[allow(dead_code)]
const INPUT: &'static str = "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

#[allow(dead_code)]
const EXPECTED: &'static str = "46";

fn main() {
    let input = include_str!("../input.txt");
    let sol = solution(input);
    dbg!(sol);
}

#[allow(dead_code)]
fn orig_solution(maps: Vec<Vec<Vec<usize>>>, seed_ranges: Vec<(usize, usize)>) -> String {
    for i in 0.. {
        if (i % 10_000) == 0 {
            println!("{i} iterations...");
        }

        let mut v = i;
        for map in maps.iter().rev() {
            for sub_map in map {
                if sub_map[0] <= v && v < sub_map[0] + sub_map[2] {
                    v = sub_map[1] + v - sub_map[0];
                    break;
                }
            }
        }
        for seed_range in &seed_ranges {
            if v >= seed_range.0 && v < seed_range.0 + seed_range.1 {
                return format!("{}", i);
            }
        }
    }
    unreachable!()
}

#[allow(dead_code)]
fn interval_solution(maps: Vec<Vec<Vec<usize>>>, seed_ranges: Vec<(usize, usize)>) -> String {
    let mut intervals: Vec<_> = seed_ranges.iter().map(|x| (x.0, x.0 + x.1)).collect();
    for map in maps.iter() {
        let mut mapped_intervals: Vec<(usize, usize)> = Vec::new();
        while !intervals.is_empty() {
            let interval = intervals.pop().unwrap();
            let start = interval.0;
            let end = interval.1;
            let mut found = false;
            for submap in map {
                let dest = submap[0];
                let src = submap[1];
                let len = submap[2];
                // Find the overlap of the two intervals
                let overlap_start = max(start, src);
                let overlap_end = min(end, src + len);
                if overlap_start < overlap_end {
                    // Mapping interval from src to dest
                    mapped_intervals.push((overlap_start - src + dest, overlap_end - src + dest));
                    // Interval before overlap may be mapped by another interval,
                    // add back in to check.
                    if start < overlap_start {
                        intervals.push((start, overlap_start));
                    }
                    // Same for any interval after the overlap.
                    if end > overlap_end {
                        intervals.push((overlap_end, end));
                    }
                    // Can only overlap with one interval, so we can break early.
                    found = true;
                    break;
                }
            }
            // Not mapped by any interval, add a 1-to-1 mapping.
            if !found {
                mapped_intervals.push((start, end));
            }
        }
        intervals = mapped_intervals.clone();
    }

    // The solution is the first seed in the smallest interval.
    let ans = intervals.iter().min().unwrap().0;
    return format!("{}", ans);
}

fn solution(s: &str) -> String {
    let mut lines = s.lines();
    let mut maps: Vec<Vec<Vec<usize>>> = Vec::new();
    let seeds: Vec<usize> = lines.next().unwrap().split_once(": ").unwrap().1.split(" ").map(|x| x.parse::<usize>().unwrap()).collect();
    let seed_ranges: Vec<(usize, usize)> = seeds.chunks(2).map(|chunk| (chunk[0], chunk[1])).collect();

    let _ = lines.next();

    loop {
        if lines.next().is_none() {
            break;
        }
        let m: Vec<Vec<_>> = lines.by_ref().take_while(|line| !line.is_empty()).map(|line| line.split(" ").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>()).collect();
        maps.push(m);
    }
    // orig_solution(maps, seed_ranges)
    interval_solution(maps, seed_ranges)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert!(solution(INPUT) == EXPECTED.to_string());
    }
}
