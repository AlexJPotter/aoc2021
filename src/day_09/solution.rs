use std::collections::HashMap;
use itertools::Itertools;
use crate::file_helpers::read_file_as_lines;

pub fn part_1() {
    let height_map = get_height_map();

    let values = &height_map.values;
    let height = values.len() as isize;
    let width = values.first().unwrap().len() as isize;

    let mut solution = 0;

    for y in 0..(height) {
        for x in 0..(width) {
            if is_low_point(&height_map, x, y) {
                let value = get_value(&height_map, x, y).unwrap();
                let risk_level = get_risk_level(value);
                solution = solution + risk_level;
            }
        }
    }

    println!("{}", solution);
}

pub fn part_2() {
    let height_map = get_height_map();

    let values = &height_map.values;
    let height = values.len() as isize;
    let width = values.first().unwrap().len() as isize;

    let mut basin_sizes: HashMap<String, i32> = HashMap::new();

    for y in 0..(height) {
        for x in 0..(width) {
            let basin_low_point = get_basin_low_point(&height_map, x, y);

            if basin_low_point.is_some() {
                let unwrapped = basin_low_point.unwrap();
                let str_low_point = unwrapped.as_str();
                let current = basin_sizes.get(str_low_point).unwrap_or(&0);
                let next = *current + 1;
                basin_sizes.insert(str_low_point.to_string(), next);
            }
        }
    }

    let sizes = basin_sizes.values().sorted().map(|&v| v).collect::<Vec<i32>>();

    let biggest = sizes[sizes.len() - 1];
    let second_biggest = sizes[sizes.len() - 2];
    let third_biggest = sizes[sizes.len() - 3];

    let solution = biggest * second_biggest * third_biggest;

    println!("{}", solution);
}

struct HeightMap {
    values: Vec<Vec<i32>>,
}

fn get_height_map() -> HeightMap {
    let values: Vec<Vec<i32>> = read_file_as_lines("day_09/input.txt")
        .iter()
        .map(|l| {
            let heights: Vec<i32> = l.chars().map(|c| {
                let height: i32 = c.to_string().parse().expect("Invalid number");
                return height;
            }).collect::<Vec<i32>>();
            return heights;
        })
        .collect();

    return HeightMap { values };
}

fn get_value(height_map: &HeightMap, x: isize, y: isize) -> Option<i32> {
    let values = &height_map.values;
    let row_count = values.len() as isize;

    if y < 0 || y >= row_count {
        return Option::None;
    }

    let row = &values[y as usize];
    let column_count = row.len() as isize;

    if x < 0 || x >= column_count {
        return Option::None;
    }

    return Option::from(row[x as usize]);
}

fn get_adjacent_values(height_map: &HeightMap, x: isize, y: isize) -> Vec<i32> {
    let left = (x - 1, y);
    let right = (x + 1, y);
    let up = (x, y - 1);
    let down = (x, y + 1);

    return [left, right, up, down]
            .map(|c| get_value(height_map, c.0, c.1))
            .iter()
            .filter(|&v| v.is_some())
            .map(|&v| v.unwrap())
            .collect::<Vec<i32>>();
}

fn get_risk_level(value: i32) -> i32 {
    return value + 1;
}

fn is_low_point(height_map: &HeightMap, x: isize, y: isize) -> bool {
    let value = get_value(&height_map, x, y).unwrap();
    let adjacent_values = get_adjacent_values(&height_map, x, y);
    return adjacent_values.iter().all(|&av| value < av);
}

fn get_basin_low_point(height_map: &HeightMap, x: isize, y: isize) -> Option<String> {
    if is_low_point(&height_map, x, y) {
        return Option::from(get_location_string(x, y)); // Is it's own basin low point
    }

    let value = get_value(height_map, x, y).unwrap();

    if value == 9 {
        return Option::None; // Locations of height 9 do not count as being in any basin
    }

    let left = get_value(&height_map, x - 1, y);
    let right = get_value(&height_map, x + 1, y);
    let up = get_value(&height_map, x, y - 1);
    let down = get_value(&height_map, x, y + 1);

    if left.unwrap_or(9) < value {
        return get_basin_low_point(&height_map, x - 1, y);
    }

    if right.unwrap_or(9) < value {
        return get_basin_low_point(&height_map, x + 1, y);
    }

    if up.unwrap_or(9) < value {
        return get_basin_low_point(&height_map, x, y - 1);
    }

    if down.unwrap_or(9) < value {
        return get_basin_low_point(&height_map, x , y + 1);
    }

    panic!("Couldn't get basin low point");
}

fn get_location_string(x: isize, y: isize) -> String {
    return format!("{},{}", x, y);
}
