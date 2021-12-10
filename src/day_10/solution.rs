use itertools::Itertools;
use crate::file_helpers::read_file_as_lines;

pub fn part_1() {
    let solution: u64 =
        read_file_as_lines("day_10/input.txt")
            .iter()
            .map(|l| get_line_score(l))
            .sum();

    println!("{}", solution);
}

pub fn part_2() {
    let scores =
        read_file_as_lines("day_10/input.txt")
            .iter()
            .filter(|l| get_line_score(l) == 0)
            .map(|l| get_missing_score(l))
            .sorted()
            .collect::<Vec<u64>>();

    let length = scores.len();
    let solution = scores[length / 2];
    println!("{}", solution);
}

fn get_missing_braces(line: &String) -> Vec<char> {
    let mut char_stack: Vec<char> = Vec::new();

    for c in line.chars() {
        if is_close_brace(&c) {
            let previous_character = char_stack.last();
            let expected = get_matching_close_brace(previous_character.unwrap());

            if c != expected {
                panic!("Found mismatched closing brace");
            } else {
                char_stack.pop();
            }
        } else {
            char_stack.push(c);
        }
    }

    char_stack.reverse();

    return char_stack.iter().map(|c| get_matching_close_brace(c)).collect();
}

fn get_missing_score(line: &String) -> u64 {
    let missing_braces  = get_missing_braces(line);

    let mut score: u64 = 0;

    for c in missing_braces {
        let char_score = get_missing_char_score(&c);
        score = (score * 5) + char_score;
    }

    return score;
}

fn get_line_score(line: &String) -> u64 {
    let mut char_stack: Vec<char> = Vec::new();

    for c in line.chars() {
        let previous_character = char_stack.last();

        if is_close_brace(&c) {
            let expected = get_matching_close_brace(previous_character.unwrap());

            if c != expected {
                return get_char_score(&c);
            } else {
                char_stack.pop();
            }
        } else {
            char_stack.push(c);
        }
    }

    return 0;
}

fn is_close_brace(c: &char) -> bool {
    return match c {
        ')' => true,
        ']' => true,
        '}' => true,
        '>' => true,
        _ => false,
    }
}

fn get_matching_close_brace(c: &char) -> char {
    return match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!("Could not get matching close brace"),
    }
}

fn get_char_score(c: &char) -> u64 {
    return match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("Could not get score"),
    }
}

fn get_missing_char_score(c: &char) -> u64 {
    return match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => panic!("Could not get missing score"),
    }
}
