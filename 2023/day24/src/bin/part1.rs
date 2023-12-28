#[allow(dead_code)]
const INPUT: &'static str = "\
19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";

#[allow(dead_code)]
const EXPECTED: &'static str = "2";


fn main() {
    let input = include_str!("../input.txt");
    let sol = solution(input);
    dbg!(sol);
}

fn solution(s: &str) -> String {
    let search_min = 200000000000000.0;
    let search_max = 400000000000000.0;
    let hail: Vec<Vec<Vec<f64>>> = s.lines().map(|line| line.split(" @ ").map(|points| points.split(",").map(|p| p.trim().parse().unwrap()).collect()).collect()).collect();
    let hail_meta: Vec<(f64, f64)> = hail.iter().map(|h| {
        // b = y - mx
        let m = h[1][1] / h[1][0]; 
        let b = h[0][1] - m * h[0][0];
        (m, b)

    }).collect();

    // Find intersection by solving system of linear equations.
    let mut ans = 0;
    for i in 1..hail.len() {
        for j in 0..i {
            let (m1, b1) = hail_meta[i];
            let (m2, b2) = hail_meta[j];
            println!("Hail {}: {:?}", i, hail[i]);
            println!("Hail {}: {:?}", j, hail[j]);
            if m1 == m2 {
                println!("Parallel, no intersect");
                continue;
            }

            let x = (b1 - b2) / (m2 - m1);
            let y = m1 * x + b1;

            let x1 = hail[i][0][0];
            let dx1 = hail[i][1][0];
            let x2 = hail[j][0][0];
            let dx2 = hail[j][1][0];
            let is_forward_1 = dx1.is_sign_positive() == (x > x1);
            let is_forward_2 = dx2.is_sign_positive() == (x > x2);
        
            if x >= search_min && x <= search_max && y >= search_min && y <= search_max && is_forward_1 && is_forward_2 {
                println!("Intersection in search area at x={x} y={y}");
                ans += 1;
            } else {
                println!("Intersection outside search area at x={x} y={y}");
            }

        }
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
