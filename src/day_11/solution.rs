use std::collections::HashSet;
use crate::file_helpers::read_file_as_lines;

pub fn part_1() {
    let initial_energy_levels = get_initial_energy_levels();
    let (_, total_flashes) = run(initial_energy_levels, 100, 0);
    println!("{}", total_flashes);
}

pub fn part_2() {
    let initial_energy_levels = get_initial_energy_levels();
    let solution = run_2(initial_energy_levels, 0);
    println!("{}", solution);
}

fn run(energy_levels: [[u8; 10]; 10], steps_left: i32, total_flashes: usize) -> ([[u8; 10]; 10], usize) {
    if steps_left == 0 {
        return (energy_levels, total_flashes);
    }

    let energy_levels_plus_one = increase_energy(energy_levels);
    let (next_levels, new_flashes) = handle_flashes(energy_levels_plus_one);

    return run(next_levels, steps_left - 1, total_flashes + new_flashes);
}

fn run_2(energy_levels: [[u8; 10]; 10], steps_run: i32) -> i32 {
    let energy_levels_plus_one = increase_energy(energy_levels);
    let (next_levels, new_flashes) = handle_flashes(energy_levels_plus_one);

    if new_flashes == 100 {
        return steps_run + 1;
    }

    return run_2(next_levels, steps_run + 1);
}

struct Location {
    x: isize,
    y: isize,
}

fn get_initial_energy_levels() -> [[u8; 10]; 10] {
    let mut energy_levels: [[u8; 10]; 10] = [[0; 10]; 10];

    let lines = read_file_as_lines("day_11/input.txt");

    for x in 0..10 {
        for y in 0..10 {
            let row = &lines[y as usize];
            let energy_level: u8 = row.chars().collect::<Vec<char>>()[x as usize].to_digit(10).unwrap() as u8;
            energy_levels[y][x] = energy_level;
        }
    }

    return energy_levels;
}

fn increase_energy(energy_levels: [[u8; 10]; 10]) -> [[u8; 10]; 10] {
    let mut new_levels = energy_levels.clone();

    for x in 0..10 {
        for y in 0..10 {
            new_levels[y][x] = new_levels[y][x] + 1;
        }
    }

    return new_levels;
}

fn handle_flashes(energy_levels: [[u8; 10]; 10]) -> ([[u8; 10]; 10], usize) {
    let mut new_energy_levels = energy_levels.clone();
    let mut has_flashed: HashSet<String> = HashSet::new();

    let mut should_loop = true;

    while should_loop {
        should_loop = false;

        for x in 0..10 {
            for y in 0..10 {
                let location = Location { x, y };
                let loc_string = get_loc_string(&location);
                
                let energy_level = get_energy_level_at_location(new_energy_levels, &location);
                
                if energy_level.is_some() {
                    if energy_level.unwrap() > 9 && !has_flashed.contains(&loc_string) {
                        let neighbours = get_neighbours(&location);
                        
                        for neighbour in neighbours {
                            let neighbour_energy = get_energy_level_at_location(new_energy_levels, &neighbour).unwrap();
                            new_energy_levels[neighbour.y as usize][neighbour.x as usize] = neighbour_energy + 1;
                        }

                        has_flashed.insert(loc_string);
                        should_loop = true; // Only loop if something flashed again
                    }
                }
            }
        }
    }

    for loc_string in &has_flashed {
        let location = from_loc_string(&loc_string);
        new_energy_levels[location.y as usize][location.x as usize] = 0;
    }

    return (new_energy_levels, has_flashed.len());
}

fn get_energy_level_at_location(energy_levels: [[u8; 10]; 10], location: &Location) -> Option<u8> {
    let x = location.x;
    let y = location.y;

    if x < 0 || x > 9 || y < 0 || y > 9 {
        return Option::None;
    }

    return Option::from(energy_levels[y as usize][x as usize]);
}

fn get_neighbours(location: &Location) -> Vec<Location> {
    let directions: [(isize, isize); 8] =
        [(0, 1), (1, 1), (1, 0), (1, -1), (0, -1), (-1, -1), (-1, 0), (-1, 1)];

    return directions.iter().map(|d| {
        let x = location.x + d.0;
        let y = location.y + d.1;
        return Location { x, y };
    }).filter(|l| l.x >= 0 && l.x < 10 && l.y >= 0 && l.y < 10)
      .collect::<Vec<Location>>();
}

fn get_loc_string(loc: &Location) -> String {
    return format!("{},{}", loc.x, loc.y);
}

fn from_loc_string(loc_str: &String) -> Location {
    let x: isize = loc_str[0..1].parse().expect("Invalid x");
    let y: isize = loc_str[2..3].parse().expect("Invalid y");
    return Location { x, y };
}
