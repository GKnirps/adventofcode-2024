#![forbid(unsafe_code)]

use std::cmp::Ordering;
use std::collections::HashSet;
use std::env;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args()
        .nth(1)
        .ok_or_else(|| "No file name given.".to_owned())?;
    let content = read_to_string(Path::new(&filename)).map_err(|e| e.to_string())?;
    let robots = parse(&content)?;

    let safety_factor = safety_factor_after_time(&robots, 100, 101, 103);
    println!("the safety factor after 100s is {safety_factor}");

    let tree_time = find_tree_config(&robots, 101, 103);
    println!(
        "Possible tree config after {tree_time} seconds. Please double-check the image above."
    );

    Ok(())
}

fn safety_factor_after_time(robots: &[Robot], t: i64, width: i64, height: i64) -> u64 {
    let (q1, q2, q3, q4) = robots
        .iter()
        .copied()
        .map(|robot| move_robot(robot, t, width, height))
        .fold(
            (0u64, 0u64, 0u64, 0u64),
            |(mut q1, mut q2, mut q3, mut q4), robot| {
                match (robot.px.cmp(&(width / 2)), robot.py.cmp(&(height / 2))) {
                    (Ordering::Less, Ordering::Less) => q1 += 1,
                    (Ordering::Less, Ordering::Greater) => q2 += 1,
                    (Ordering::Greater, Ordering::Less) => q3 += 1,
                    (Ordering::Greater, Ordering::Greater) => q4 += 1,
                    _ => (),
                }
                (q1, q2, q3, q4)
            },
        );
    q1 * q2 * q3 * q4
}

fn move_robot(robot: Robot, t: i64, width: i64, height: i64) -> Robot {
    let px = (robot.px + t * robot.vx).rem_euclid(width);
    let py = (robot.py + t * robot.vy).rem_euclid(height);
    Robot { px, py, ..robot }
}

fn print_robots(robot_pos: &HashSet<(i64, i64)>, width: i64, height: i64) {
    for y in 0..height {
        for x in 0..width {
            if robot_pos.contains(&(x, y)) {
                print!("R");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn n_robots_in_a_row(robot_pos: &HashSet<(i64, i64)>, n: i64) -> bool {
    robot_pos
        .iter()
        .any(|(px, py)| (1..=n).all(|dx| robot_pos.contains(&(px + dx, *py))))
}

fn find_tree_config(robots: &[Robot], width: i64, height: i64) -> i64 {
    let mut robot_pos: HashSet<(i64, i64)> =
        robots.iter().map(|robot| (robot.px, robot.py)).collect();
    let mut time = 0;
    while !n_robots_in_a_row(&robot_pos, 30) {
        time += 1;
        robot_pos.clear();
        for robot in robots {
            let trobot = move_robot(*robot, time, width, height);
            robot_pos.insert((trobot.px, trobot.py));
        }
    }
    print_robots(&robot_pos, width, height);
    time
}

#[derive(Copy, Clone, Debug)]
struct Robot {
    px: i64,
    py: i64,
    vx: i64,
    vy: i64,
}

fn parse(input: &str) -> Result<Box<[Robot]>, String> {
    input.lines().map(parse_robot).collect()
}

fn parse_robot(line: &str) -> Result<Robot, String> {
    let (p, v) = line
        .split_once(' ')
        .ok_or_else(|| format!("unable to split position from velocity in line '{line}'"))?;

    let (px, py) = p
        .strip_prefix("p=")
        .and_then(|p| p.split_once(','))
        .ok_or_else(|| format!("invalid format for position in line '{line}'"))?;
    let px: i64 = px
        .parse()
        .map_err(|e| format!("unable to parse px in line '{line}': {e}"))?;
    let py: i64 = py
        .parse()
        .map_err(|e| format!("unable to parse py in line '{line}': {e}"))?;

    let (vx, vy) = v
        .strip_prefix("v=")
        .and_then(|v| v.split_once(','))
        .ok_or_else(|| format!("invalid format for velocity in line '{line}'"))?;
    let vx: i64 = vx
        .parse()
        .map_err(|e| format!("unable to parse vx in line '{line}': {e}"))?;
    let vy: i64 = vy
        .parse()
        .map_err(|e| format!("unable to parse vy in line '{line}': {e}"))?;

    Ok(Robot { px, py, vx, vy })
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &str = r#"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
"#;

    #[test]
    fn safety_factor_after_time_works_for_example() {
        // given
        let robots = parse(EXAMPLE).expect("expected example to parse");

        // when
        let sf = safety_factor_after_time(&robots, 100, 11, 7);

        // then
        assert_eq!(sf, 12);
    }
}
