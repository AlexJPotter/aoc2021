use crate::file_helpers::read_file_as_lines;
use std::collections::HashMap;

pub fn part_1() {
    let lines = get_lines();

    let mut straight_lines: Vec<Line> = Vec::new();

    lines
        .iter()
        .filter(|&line| line.step.x == 0 || line.step.y == 0)
        .for_each(|&line| straight_lines.push(line.clone()));

    let point_counts = get_point_counts(&straight_lines);

    let dangerous_points = point_counts.iter().filter(|&pair| *pair.1 > 1).count();

    println!("{}", dangerous_points);
}

pub fn part_2() {
    let lines = get_lines();
    let point_counts = get_point_counts(&lines);
    let dangerous_points = point_counts.iter().filter(|&pair| *pair.1 > 1).count();
    println!("{}", dangerous_points);
}

fn get_point_counts(lines: &Vec<Line>) -> HashMap<String, u32> {
    let mut point_counts: HashMap<String, u32> = HashMap::new();

    for line in lines {
        let points = get_points(&line);

        for point in points {
            let str_point = print_vec2(&point);

            let point_count = point_counts.get(&str_point);
            let new_count = if point_count.is_none() { 1 } else { point_count.unwrap() + 1 };
            point_counts.insert(str_point, new_count);
        }
    }

    return point_counts;
}

#[derive(PartialEq, Eq, Copy, Clone)]
struct Vector2 {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy)]
struct Line {
    start: Vector2,
    end: Vector2,
    step: Vector2,
}

fn get_lines() -> Vec<Line> {
    return read_file_as_lines("day_05/input.txt")
            .iter()
            .map(|str_line| parse_line(str_line))
            .collect::<Vec<Line>>();
}

fn parse_line(str_line: &String) -> Line {
    let parts = str_line.split(" -> ").collect::<Vec<&str>>();
    let left = parts[0];
    let right = parts[1];
    let start = parse_vector(left);
    let end = parse_vector(right);
    let absolute_direction = difference(&start, &end);
    let scale_factor = num::Integer::gcd(&absolute_direction.x, &absolute_direction.y);
    let step = divide_by(&absolute_direction, scale_factor);
    let line = Line { start, end, step };
    return line;
}

fn parse_vector(coordinates: &str) -> Vector2 {
    let parts = coordinates.split(",").collect::<Vec<&str>>();
    let x: i32 = parts[0].parse().expect("Invalid coordinate");
    let y: i32 = parts[1].parse().expect("Invalid coordinate");
    return Vector2 { x, y };
}

fn add(val1: &Vector2, val2: &Vector2) -> Vector2 {
    let x = val1.x + val2.x;
    let y = val1.y + val2.y;
    return Vector2 { x, y};
}

fn difference(val1: &Vector2, val2: &Vector2) -> Vector2 {
    let x = val2.x - val1.x;
    let y = val2.y - val1.y;
    return Vector2 { x, y };
}

fn divide_by(vec2: &Vector2, divisor: i32) -> Vector2 {
    return Vector2 { x: vec2.x / divisor, y: vec2.y / divisor };
}

fn print_vec2(vec2: &Vector2) -> String {
    return format!("{},{}", vec2.x.to_string(), vec2.y.to_string());
}

fn get_points(line: &Line) -> Vec<Vector2> {
    let mut points: Vec<Vector2> = Vec::new();
    let mut current = line.start;

    points.push(line.start.clone());

    while current != line.end {
        current = add(&current, &line.step);
        points.push(current.clone());
    }

    return points;
}
