#[allow(dead_code)]
const INPUT: &'static str = "\
rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

#[allow(dead_code)]
const EXPECTED: &'static str = "1320";

fn main() {
    let input = include_str!("../input.txt");
    let sol = solution(input);
    dbg!(sol);
}

fn solution(s: &str) -> String {
    let ans: usize = s.lines().next().unwrap().split(",").fold(0, |acc, seq| {
        let mut curr = 0;
        for c in seq.chars() {
            curr += c as usize;
            curr *= 17;
            curr %= 256;
        }
        acc + curr
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
