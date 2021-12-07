use crate::file_helpers::read_file_as_string;

pub fn part_1() {
    let input = get_input();
    let values_to_try: Vec<i32> = get_values_to_try(&input);
    let solution: i32 = values_to_try.iter().map(|&v| get_cost(v, &input)).min().unwrap();
    println!("{}", solution.to_string());
}

pub fn part_2() {
    let input = get_input();
    let values_to_try: Vec<i32> = get_values_to_try(&input);
    let solution: i32 = values_to_try.iter().map(|&v| get_cost_2(v, &input)).min().unwrap();
    println!("{}", solution.to_string());
}

fn get_cost(p: i32, ps: &Vec<i32>) -> i32 {
    return ps.iter().map(|&x| (x - p).abs()).sum();
}

fn get_cost_2(p: i32, ps: &Vec<i32>) -> i32 {
    return ps.iter().map(|&x| {
        let d = (x - p).abs();
        return get_distance_cost(d);
    }).sum();
}

fn get_distance_cost(d: i32) -> i32 {
    let float_r = 0.5 * (d as f32) * ((d + 1) as f32);
    return float_r as i32;
}

fn get_input() -> Vec<i32> {
    return read_file_as_string("day_07/input.txt")
        .trim()
        .split(",")
        .filter(|s| s.trim() != "")
        .map(|s| {
            let s_num: i32 = s.trim().parse().expect("Invalid number");
            return s_num;
        })
        .collect::<Vec<i32>>();
}

fn get_values_to_try(input: &Vec<i32>) -> Vec<i32> {
    let &min = input.iter().min().unwrap();
    let &max = input.iter().max().unwrap();
    return (min..max).collect::<Vec<i32>>();
}
