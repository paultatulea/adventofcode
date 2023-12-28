#[allow(dead_code)]
const INPUT: &'static str = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

#[allow(dead_code)]
const EXPECTED: &'static str = "8";

fn main() {
    let input = include_str!("../input.txt");
    let sol = solution(input);
    dbg!(sol);
}

fn solution(s: &str) -> String {
    let ans: u32 = s.lines().map(|line| {
        let parts = line.split_once(":").unwrap();
        let game_id = parts.0[5..].parse::<u32>().unwrap();
        if parts.1.split(";").all(|sg| {
            sg.split(",").all(|cube| {
                let mut iter = cube.trim().split(" ");
                let num = iter.next().unwrap().parse::<u32>().unwrap();
                let color = iter.next().unwrap();
                let ok = num <= match color {
                    "red" => 12,
                    "green" => 13,
                    "blue" => 14,
                    _ => panic!("Unhandled color: {color}"),
                };
                ok
            })
        }) {
            game_id
        } else {
            0
        }
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
