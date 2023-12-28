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
const EXPECTED: &'static str = "35";

fn main() {
    let input = include_str!("../input.txt");
    let sol = solution(input);
    dbg!(sol);
}

fn solution(s: &str) -> String {
    let mut lines = s.lines();
    let mut maps: Vec<Vec<Vec<usize>>> = Vec::new();
    let seeds: Vec<usize> = lines.next().unwrap().split_once(": ").unwrap().1.split(" ").map(|x| x.parse::<usize>().unwrap()).collect();

    let _ = lines.next();

    loop {
        if lines.next().is_none() {
            break;
        }
        let m: Vec<Vec<_>> = lines.by_ref().take_while(|line| !line.is_empty()).map(|line| line.split(" ").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>()).collect();
        maps.push(m);
    }
    println!("{:?}", maps);

    let ans = seeds.iter().map(|seed| {
        let mut v = *seed;
        for map in &maps {
            for sub_map in map {
                if sub_map[1] <= v && v < sub_map[1] + sub_map[2] {
                    v = sub_map[0] + v - sub_map[1];
                    break;
                }
            }
        }
        v
    }).min().unwrap();
    format!("{}", ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert!(solution(INPUT) == EXPECTED.to_string());
    }
}
