use std::io;

use itertools::Itertools;
use nalgebra::Vector3;
use num_integer::Integer;
use num_rational::Ratio;

#[derive(Debug)]
struct Hailstone {
    x: i64,
    y: i64,
    z: i64,
    vx: i32,
    vy: i32,
    vz: i32,
}

const AREA_MIN: i128 = 200000000000000i128;
const AREA_MAX: i128 = 400000000000000i128;

fn cross_x(
    slope_a: Ratio<i128>,
    slope_b: Ratio<i128>,
    y_a: i64,
    x_a: i64,
    y_b: i64,
    x_b: i64,
) -> Ratio<i128> {
    (Ratio::from_integer((y_b - y_a) as i128) + slope_a * x_a as i128 - slope_b * x_b as i128) / (slope_a - slope_b)
}

fn part1(hailstones: &[Hailstone]) -> usize {
    let mut count = 0;
    let area_min: Ratio<i128> = AREA_MIN.into();
    let area_max: Ratio<i128> = AREA_MAX.into();

    for (a, b) in hailstones.iter().tuple_combinations() {
        let slope_a: Ratio<i128> = Ratio::new(a.vy.into(), a.vx.into());
        let slope_b: Ratio<i128> = Ratio::new(b.vy.into(), b.vx.into());

        if slope_a == slope_b {
            continue;
        }

        let x = cross_x(slope_a, slope_b, a.y, a.x, b.y, b.x);
        if x >= area_min && x <= area_max {
            if ((x - Ratio::from_integer(a.x as i128)).numer() > &0) != (a.vx > 0) 
              || ((x - Ratio::from_integer(b.x as i128)).numer() > &0) != (b.vx > 0) {
                continue;
            }

            let y = slope_a * x + a.y as i128 - slope_a * a.x as i128;

            
            if y >= area_min && y <= area_max {
                count += 1;
            }
        }
    }

    count
}

fn find_intercept_time(l: Vector3<i128>, lp: Vector3<i128>, hv: Vector3<i128>, hp: Vector3<i128>) -> (i128, i128) {
    // Solve:
    // ip_x + t * l_x = d_pos_x + s * d_vel_x  <=>  t*l_x - s*d_vel_x = d_pos_x - ip_x
    // ip_y + t * l_y = d_pos_y + s * d_vel_y  <=>  t*l_y - s*d_vel_y = d_pos_y - ip_y
    // (ip_z + t * l_z = d_pos_z + s * d_vel_z  <=>  t*l_z - s*d_vel_z = d_pos_z - ip_z)

    // Cramer's rule
    let b = hp - lp;
    let det_a = l.x * -hv.y + hv.x * l.y;
    let det_a_t = b.x * -hv.y + hv.x * b.y;
    let det_a_s = l.x * b.y - b.x * l.y;

    let t = det_a_t / det_a;
    let s = det_a_s / det_a;

    assert_eq!(lp + l * t, hp + hv * s);
    assert!(s >= 0);

    let init_pos = lp + l*t - l*s;
    assert_eq!(init_pos + l*s, hp + hv*s);

    (t, s)
}


fn part2(hailstones: &[Hailstone]) -> i128 {

    let a = &hailstones[0];
    let b = &hailstones[1];
    let c = &hailstones[3];
    let d = &hailstones[4];

    let a_pos = Vector3::new(a.x as i128, a.y as i128, a.z as i128);
    let a_vel = Vector3::new(a.vx as i128, a.vy as i128, a.vz as i128);
    
    let b_vel = Vector3::new(b.vx as i128, b.vy as i128, b.vz as i128);
    let b_pos_1 = Vector3::new(b.x as i128, b.y as i128, b.z as i128);
    let b_pos_2 = b_pos_1 + b_vel * 100;

    let c_vel = Vector3::new(c.vx as i128, c.vy as i128, c.vz as i128);
    let c_pos = Vector3::new(c.x as i128, c.y as i128, c.z as i128);

    let d_vel = Vector3::new(d.vx as i128, d.vy as i128, d.vz as i128);
    let d_pos = Vector3::new(d.x as i128, d.y as i128, d.z as i128);

    let get_dist_at_time = |t: i128| {
        // Find a's position at timestamp t
        let ap = a_vel * t + a_pos;
        
        // Using a's position and two arbitrary positions of b, form a plane
        let plane_normal = (b_pos_1 - ap).cross(&(b_pos_2 - ap));

        // Find the intersection point of the c line and the new plane
        let d = (ap - c_pos).dot(&plane_normal) as f64 / (c_vel.dot(&plane_normal)) as f64;
        let ip: Vector3<f64> = c_pos.cast() + c_vel.cast() * d;

        // Form line (vector) from a's position to the intersection point
        let l = ip - ap.cast();

        // Direction vector of line connecting closest points between l and d
        let n = l.cross(&d_vel.cast());
        
        // Distance between the line through ap-ip and the d line
        (n.dot(&(ip - d_pos.cast()))).abs() / n.magnitude()
    };

    let mut t_start = 0;
    let mut t_end = 4_000_000_000_000;
    let mut step = 10_000_000_000;

    // Iterate through t until we find a local minimum
    let t_target = loop {
        let ((t_min, _), _) = (t_start..t_end).step_by(step).map(|t| (t, get_dist_at_time(t)))
        .tuple_windows()
        .find(|(a, b)| b.1 > a.1)
        .unwrap();
        if step > 1 {
            t_start = t_min - step as i128;
            t_end = t_min + step as i128;
            step = 1.max(step / 100);
        } else {
            break t_min
        }
    };


    // Calculate the integer d and the vector of the line that crosses all other lines
    // (the minimum finding above uses floats)

    // A's position at t_target
    let ap = a_vel * t_target + a_pos;
    // Plane formed by ap, and two b positions at arbitrary times
    let plane_normal = (b_pos_1 - ap).cross(&(b_pos_2 - ap));

    let (d, rem) = (ap - c_pos).dot(&plane_normal).div_rem(&(c_vel.dot(&plane_normal)));
    assert_eq!(rem, 0);
    let lp = c_pos + c_vel * d;
    
    // The direction vector of the throw line
    let l = lp - ap;
    let gcd = l.x.gcd(&l.y).gcd(&l.z);
    let mut l = l / gcd;

    // Flip vector l around (negate) if necessary so that time moves
    // linearly forwards
    let (a_t, a_s) = find_intercept_time(l, lp, a_vel, a_pos);
    let (d_t, d_s) = find_intercept_time(l, lp, d_vel, d_pos);
    if (d_s - a_s).signum() != (d_t - a_t).signum() {
        l.neg_mut();
    }

    // Find the time that line d and our throw line crosses
    let (d_t, d_s) = find_intercept_time(l, lp, d_vel, d_pos);
    // Use that to find the initial position
    let init_pos = lp + l*d_t -  l*d_s;

    init_pos.x + init_pos.y + init_pos.z
}

fn parse_input(input: Vec<String>) -> Vec<Hailstone> {
    input
        .into_iter()
        .map(|l| {
            let (x, y, z, vx, vy, vz) = aoclib::read_ints_from_string::<i64>(&l, true)
                .into_iter()
                .collect_tuple()
                .unwrap();
            Hailstone {
                x,
                y,
                z,
                vx: vx as i32,
                vy: vy as i32,
                vz: vz as i32,
            }
        })
        .collect()
}

fn main() -> io::Result<()> {
    let input = aoclib::read_input_lines()?;
    let input = parse_input(input);

    let p1 = part1(&input);
    println!("Part 1: {}", p1);

    let p2 = part2(&input);
    println!("Part 2: {}", p2);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_real_input() {
        let input = aoclib::read_file_lines(aoclib::get_test_input_file!(24)).unwrap();
        let input = parse_input(input);

        let p1 = part1(&input);
        assert_eq!(p1, 18098);

        let p2 = part2(&input);
        assert_eq!(p2, 886858737029295);
    }
}
