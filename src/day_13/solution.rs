use std::collections::HashSet;
use itertools::Itertools;
use crate::file_helpers::read_file_as_lines;

pub fn part_1() {
    let (positions, folds) = get_input();

    let first_fold = folds.first().unwrap();

    let after_fold = fold(&positions, first_fold);
    println!("Before: {}; After: {}", positions.len().to_string(), after_fold.len().to_string());
}

pub fn part_2() {
    let (initial_positions, folds) = get_input();

    let final_positions =
        folds.iter()
             .fold(initial_positions, |positions, current_fold| fold(&positions, current_fold));

    draw(&final_positions);
}

type Position = (u16, u16);

type Fold = (char, u16);

fn fold(positions: &Vec<Position>, fold: &Fold) -> Vec<Position> {
    let (axis, along) = fold;

    return match axis {
        'x' => fold_x(positions, *along),
        'y' => fold_y(positions, *along),
        _ => panic!("Could not fold"),
    }
}

fn fold_x(positions: &Vec<Position>, along_x: u16) -> Vec<Position> {
    return
        positions.iter()
                 .map(|p| fold_pos_x(p, along_x))
                 .unique()
                 .collect_vec();
}

fn fold_pos_x(position: &Position, along_x: u16) -> Position {
    let (x_before, y_before) = *position;

    if x_before < along_x {
        return *position;
    }

    let x_after = along_x - (x_before - along_x);
    let y_after = y_before;

    return (x_after, y_after);
}

fn fold_y(positions: &Vec<Position>, along_y: u16) -> Vec<Position> {
    return
        positions.iter()
            .map(|p| fold_pos_y(p, along_y))
            .unique()
            .collect_vec();
}

fn fold_pos_y(position: &Position, along_y: u16) -> Position {
    let (x_before, y_before) = *position;

    if y_before < along_y {
        return *position;
    }

    let x_after = x_before;
    let y_after = along_y - (y_before - along_y);

    return (x_after, y_after);
}

fn get_input() -> (Vec<Position>, Vec<Fold>) {
    let mut positions: Vec<Position> = Vec::new();
    let mut folds: Vec<Fold> = Vec::new();

    read_file_as_lines("day_13/input.txt").iter()
        .filter(|&l| l.trim() != "")
        .for_each(|l| {
            if l.starts_with("fold along") {
                folds.push(parse_fold(l));
            } else {
                positions.push(position_from_string(l));
            }
        });

    return (positions, folds);
}

fn position_from_string(string_val: &String) -> Position {
    let parts = string_val.split(",").collect_vec();
    let x: u16 = parts[0].parse().expect("Invalid x");
    let y: u16 = parts[1].parse().expect("Invalid y");
    return (x, y);
}

fn position_to_string(position: &Position) -> String {
    let (x, y) = position;
    return format!("{},{}", x.to_string(), y.to_string());
}

fn parse_fold(line: &String) -> Fold {
    let parts_1 = line.split(" ").collect_vec();
    let parts_2 = parts_1[2].split("=").collect_vec();
    let axis = parts_2[0].chars().exactly_one().unwrap();
    let place: u16 = parts_2[1].parse().expect("Invalid folding place");
    return (axis, place);
}

fn draw(positions: &Vec<Position>) {
    println!();
    let max_x = positions.iter().map(|(x, _)| *x).max().unwrap();
    let max_y = positions.iter().map(|(_, y)| *y).max().unwrap();

    let string_positions =
        positions.iter()
                 .map(|p| position_to_string(p))
                 .collect::<HashSet<String>>();

    for y in 0..=max_y {
        for x in 0..=max_x {
            let string_position = position_to_string(&(x, y));
            print!("{}", if string_positions.contains(&string_position) { "##" } else { "  " });
        }
        print!("\n");
    }
    println!();
}
