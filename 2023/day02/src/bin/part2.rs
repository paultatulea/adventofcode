use std::cmp;

#[allow(dead_code)]
const INPUT: &'static str = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

#[allow(dead_code)]
const EXPECTED: &'static str = "2286";

fn main() {
    let input = include_str!("../input.txt");
    let sol = solution(input);
    dbg!(sol);
}

fn solution(s: &str) -> String {
    let ans: u32 = s.lines().map(|line| {
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;
        line.split_once(":").unwrap().1.split(";").for_each(|sg| {
            sg.split(",").for_each(|cube| {
                let mut iter = cube.trim().split(" ");
                let num = iter.next().unwrap().parse::<u32>().unwrap();
                let color = iter.next().unwrap();
                if color == "red" {
                    max_red = cmp::max(max_red, num);
                } else if color == "green" {
                    max_green = cmp::max(max_green, num);
                } else {
                    max_blue = cmp::max(max_blue, num);
                }
            });
        });
        max_red * max_green * max_blue
    }).sum();
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
