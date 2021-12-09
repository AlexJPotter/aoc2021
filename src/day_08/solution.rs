use std::collections::HashMap;
use itertools::Itertools;
use crate::file_helpers::read_file_as_lines;

pub fn part_1() {
    let input_lines = parse_input();

    let mut count = 0;

    for line in input_lines {
        for signal in line.output {
            let unique_sizes: [usize; 4] = [2, 3, 4, 7]; // [1, 4, 7, 8];
            let len = signal.len();
            if unique_sizes.contains(&len) {
                count = count + 1;
            }
        }
    }

    println!("{}", count.to_string());
}

pub fn part_2() {
    let mut solution: i32 = 0;

    let lines = parse_input();

    for line in lines {
        let mut possible_wirings: Vec<[char; 7]> = get_possible_wirings(); // [['d','e','a','f','g','b','c']].to_vec();

        // let mut possible_segments_by_wire: HashMap<char, Vec<char>> = HashMap::new();

        // ['a', 'b', 'c', 'd', 'e', 'f', 'g'] // wires
        //     .iter()
        //     .for_each(|&wire| {
        //         possible_segments_by_wire.insert(wire, Vec::new());
        //     });
        //
        // for wiring_pattern in &line.patterns {
        //     let pattern_length = wiring_pattern.len(); // e.g. 3
        //     let wires: Vec<char> = wiring_pattern.chars().collect(); // e.g. ('d', 'b', 'g')
        //
        //     // e.g. ('a', 'c', 'f')
        //     let relevant_segments: Vec<char> = match pattern_length {
        //         2 => ['c', 'f'].to_vec(),
        //         3 => ['a', 'c', 'f'].to_vec(),
        //         4 => ['b', 'c', 'd', 'f'].to_vec(),
        //         _ => Vec::new(),
        //     };
        //
        //     for wire in &wires {
        //         for &segment in &relevant_segments {
        //             let current = possible_segments_by_wire.get(wire).unwrap();
        //             let mut next = current.clone();
        //             next.push(segment);
        //             possible_segments_by_wire.insert(*wire, next);
        //         }
        //     }
        // }

        // println!("Possible segments by wire:");
        // for l in &letters {
        //     println!("{}: {}", l, possible_segments_by_wire.get(l).unwrap().iter().collect::<String>());
        // }

        // let total_possibilities = possible_wirings.len();
        // println!("Total possibilities: {}", total_possibilities);

        // let mut reduced_possibilities: Vec<[char; 7]> = Vec::new();
        //
        // for wiring in &possible_wirings {
        //     let mut is_valid = true;
        //
        //     println!("Considering wiring: {}", wiring.iter().collect::<String>());
        //
        //     for index in 0..6 {
        //         let wire = &letters[index]; // 0 -> 'a'
        //         let segment = &wiring[index]; // 0 -> e.g. 'c'
        //         let possible_segments = possible_segments_by_wire.get(wire).unwrap();
        //
        //         println!("Wire: {}; Segment: {}; Valid segments: {}", wire, segment, possible_segments.iter().collect::<String>());
        //
        //         if !possible_segments.is_empty() {
        //             if !possible_segments.contains(segment) {
        //                 println!("Not valid");
        //                 is_valid = false;
        //             }
        //         }
        //     }
        //
        //     if is_valid {
        //         reduced_possibilities.push(wiring.clone());
        //         println!("Valid wiring: {}", wiring.iter().collect::<String>());
        //     } else {
        //         println!("! INVALID wiring: {}", wiring.iter().collect::<String>());
        //     }
        // }
        //
        // let total_reduced_possibilities = reduced_possibilities.len();
        // println!("Reduced possibilities: {}", total_reduced_possibilities);

        let mut reduced_again: Vec<[char; 7]> = Vec::new();

        for wiring in &possible_wirings {
            let mut is_valid = true;

            for wire_pattern in &line.patterns {
                let segment_pattern = to_segment_pattern(wire_pattern, wiring);
                let number = to_number(&segment_pattern);
                if number.is_none() {
                    is_valid = false;
                }
            }

            if is_valid {
                reduced_again.push(wiring.clone());
            }
        }

        let wiring = reduced_again.first().unwrap();

        let digits =
            &line.output
                .iter()
                .map(|wp| to_segment_pattern(wp, wiring))
                .map(|sp| to_number(&sp).unwrap())
                .collect::<Vec<i32>>();

        let output_value = (1000 * &digits[0]) + (100 * &digits[1]) + (10 * &digits[2]) + &digits[3];
        solution = solution + output_value;
    }

    println!("Solution {}", solution);
}

fn get_possible_wirings() -> Vec<[char; 7]> {
    return ['a', 'b', 'c', 'd', 'e', 'f', 'g']
        .iter()
        .permutations(7)
        .map(|p| [*p[0], *p[1], *p[2], *p[3], *p[4], *p[5], *p[6]])
        .collect::<Vec<[char; 7]>>();
}

struct InputLine {
    patterns: Vec<String>,
    output: Vec<String>,
}

fn parse_input() -> Vec<InputLine> {
    return read_file_as_lines("day_08/input.txt").iter()
        .map(|line| {
            let patterns = line.trim().split(" | ").collect::<Vec<&str>>()[0].trim().split(" ").map(|s| s.to_string()).collect::<Vec<String>>();
            let output = line.trim().split(" | ").collect::<Vec<&str>>()[1].trim().split(" ").map(|s| s.to_string()).collect::<Vec<String>>();
            return InputLine { patterns, output };
        })
        .collect::<Vec<InputLine>>();
}

fn to_segment_pattern(wire_pattern: &String, wiring: &[char; 7]) -> String {
    let letters = ['a', 'b', 'c', 'd', 'e', 'f', 'g'];

    return wire_pattern
        .chars()
        .map(|wire| {
            let wire_index = wiring.iter().position(|&l| l == wire).unwrap();
            let segment = letters[wire_index];
            // println!("wire {}; index {}; -> segment {}", wire, wire_index, segment);
            return segment;
        })
        .collect::<String>();;
}

fn to_number(segment_pattern: &String) -> Option<i32> {
    let normalised_segment_pattern = segment_pattern.chars().sorted().collect::<String>();

    return match normalised_segment_pattern.as_ref() {
        "abcefg" => Option::from(0),
        "cf" => Option::from(1),
        "acdeg" => Option::from(2),
        "acdfg" => Option::from(3),
        "bcdf" => Option::from(4),
        "abdfg" => Option::from(5),
        "abdefg" => Option::from(6),
        "acf" => Option::from(7),
        "abcdefg" => Option::from(8),
        "abcdfg" => Option::from(9),
        _ => Option::None,
    };
}
