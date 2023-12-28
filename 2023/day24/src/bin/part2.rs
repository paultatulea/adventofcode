use nalgebra::matrix;
use nalgebra::vector;

#[allow(dead_code)]
const INPUT: &'static str = "\
19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";

#[allow(dead_code)]
const EXPECTED: &'static str = "47";

fn main() {
    let input = include_str!("../input.txt");
    let sol = solution(input);
    dbg!(sol);
}

fn dot_product(a: &Vec<f64>, b: &Vec<f64>) -> f64 {
    a.iter().zip(b.iter()).fold(0.0, |acc, (x1, x2)| acc + x1 * x2)
}

fn cross_product(a: &Vec<f64>, b: &Vec<f64>) -> Vec<f64> {
    let x1y2 = a[0]*b[1];
    let y1z2 = a[1]*b[2];
    let z1x2 = a[2]*b[0];
    let y1x2 = a[1]*b[0];
    let z1y2 = a[2]*b[1];
    let x1z2 = a[0]*b[2];

    let x = y1z2 - z1y2;
    let y = z1x2 - x1z2;
    let z = x1y2 - y1x2;
    return vec![x, y, z];
}

fn find_non_coplanar(vecs: &Vec<Vec<Vec<f64>>>) -> Option<(usize, usize, usize)> {
    for i in 2..vecs.len() {
        for j in 1..i {
            for k in 0..j {
                if dot_product(&vecs[i][1], &cross_product(&vecs[j][1], &vecs[k][1])) != 0.0 {
                    return Some((i, j, k));
                }
            }
        }
    }
    None
}

fn solution(s: &str) -> String {
    let mut ans: i128 = 0;
    let hail: Vec<Vec<Vec<f64>>> = s.lines().map(|line| line.split(" @ ").map(|points| points.split(",").map(|p| p.trim().parse().unwrap()).collect()).collect()).collect();

    if let Some((i, j, k)) = find_non_coplanar(&hail) {
        let vec1 = &hail[i];
        let vec2 = &hail[j];
        let vec3 = &hail[k];
        println!("{i}: {vec1:?}");
        println!("{j}: {vec2:?}");
        println!("{k}: {vec3:?}");
        let x1 = vec1[0][0];
        let y1 = vec1[0][1];
        let z1 = vec1[0][2];
        let dx1 = vec1[1][0];
        let dy1 = vec1[1][1];
        let dz1 = vec1[1][2];
        let x2 = vec2[0][0];
        let y2 = vec2[0][1];
        let z2 = vec2[0][2];
        let dx2 = vec2[1][0];
        let dy2 = vec2[1][1];
        let dz2 = vec2[1][2];
        let x3 = vec3[0][0];
        let y3 = vec3[0][1];
        let z3 = vec3[0][2];
        let dx3 = vec3[1][0];
        let dy3 = vec3[1][1];
        let dz3 = vec3[1][2];

        let x_centroid = (x1 + x2 + x3) / 3.0;
        let y_centroid = (y1 + y2 + y3) / 3.0;
        let z_centroid = (z1 + z2 + z3) / 3.0;

        let x1 = x1 - x_centroid;
        let x2 = x2 - x_centroid;
        let x3 = x3 - x_centroid;
        let y1 = y1 - y_centroid;
        let y2 = y2 - y_centroid;
        let y3 = y3 - y_centroid;
        let z1 = z1 - z_centroid;
        let z2 = z2 - z_centroid;
        let z3 = z3 - z_centroid;

        /*
        * Given an equation for one hail intersecting with the rock.
        * p * v * t_i = p_i * v_i * t_i
        * Where p and v are the position and velocity vectors
        * and t_i is a scalar, the time of intersection.
        * (p - p_i) = -t_i * (v - v_i)
        * Then, since (p - p_i) is a scalar multiple (-t_i) of (v - v_i)
        * we know that the cross product is 0 of the two vectors. Each
        * component of the cross product must be equal to 0.
        * For example, take the x component of the cross product for the first and second hail.
        * (y - y1) * (dz - dz1) - (z - z1) * (dy - dy1) = 0
        * (y - y2) * (dz - dz2) - (z - z2) * (dy - dy2) = 0
        * Expand terms and note the non-linear terms can be eliminated by subtracting the second from the first.
        * Then, collect like terms with unknowns x, y, z, dx, dy, dz on the left, and constants on
        * the right.
        * (dz2 - dz1) * y + (dy2 - dy1) * z + (z2 - z1) * dy + (y2 - y1) * dz = y2 * dz2 - y1 * dz1
        * + z2 * dy2 - z1 * dy1
        * Repeating the process will result in 6 linear equations and 6 unknowns which is solvable in
        * the standard linear algebra matrix equation Ax = b.
        *
        * NOTE: There is some numerical inaccuracy here because of large numbers.
        * Truncating the result x, y, z gives me the correct answer on my data.
        * It may also be more accurate to keep all values as integers while calculating A and b, then
        * converting to float at the end.
        */

        let a = matrix![
            dy2 - dy1, dx1 - dx2, 0.0, y1 - y2, x2 - x1, 0.0;
            dy3 - dy1, dx1 - dx3, 0.0, y1 - y3, x3 - x1, 0.0;
            dz2 - dz1, 0.0, dx1 - dx2, z1 - z2, 0.0, x2 - x1;
            dz3 - dz1, 0.0, dx1 - dx3, z1 - z3, 0.0, x3 - x1;
            0.0, dz2 - dz1, dy1 - dy2, 0.0, z1 - z2, y2 - y1;
            0.0, dz3 - dz1, dy1 - dy3, 0.0, z1 - z3, y3 - y1;
        ];
        let b = vector![
            (y1 * dx1 - y2 * dx2) - (x1 * dy1 - x2 * dy2),
            (y1 * dx1 - y3 * dx3) - (x1 * dy1 - x3 * dy3),
            (z1 * dx1 - z2 * dx2) - (x1 * dz1 - x2 * dz2),
            (z1 * dx1 - z3 * dx3) - (x1 * dz1 - x3 * dz3),
            (z1 * dy1 - z2 * dy2) - (y1 * dz1 - y2 * dz2),
            (z1 * dy1 - z3 * dy3) - (y1 * dz1 - y3 * dz3),
        ];

        let result = a.lu().solve(&b).unwrap();
        println!("Result = {result:?}");
        let ans_x = result[0] + x_centroid;
        let ans_y = result[1] + y_centroid;
        let ans_z = result[2] + z_centroid;
        let trunc_ans_x = ans_x as i128;
        let trunc_ans_y = ans_y as i128;
        let trunc_ans_z = ans_z as i128;
        let ans_dx = result[3];
        let ans_dy = result[4];
        let ans_dz = result[5];
        println!("x = {ans_x}, y = {ans_y}, z = {ans_z}, dx = {ans_dx}, dy = {ans_dy}, dz = {ans_dz}]");
        println!("x_int = {trunc_ans_x}, y_int = {trunc_ans_y}, z_int = {trunc_ans_z}");
        ans = trunc_ans_x + trunc_ans_y + trunc_ans_z;
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
