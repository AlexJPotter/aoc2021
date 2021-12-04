use crate::file_helpers;

#[derive(Copy, Clone)]
struct BitCounts {
    ones: u32,
    zeros: u32,
}

pub fn part_1() {
    let input_lines = file_helpers::read_file_as_lines("day_03/input.txt");

    let mut counts_by_index: [BitCounts; 12] = [BitCounts { ones: 0, zeros: 0 }; 12];
    let indices: [usize; 12] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];

    for line in input_lines {
        for index in indices {
            let bit_char = line.chars().nth(index).unwrap();

            if bit_char == '1' {
                counts_by_index[index].ones = counts_by_index[index].ones + 1;
            } else if bit_char == '0' {
                counts_by_index[index].zeros = counts_by_index[index].zeros + 1;
            } else {
                panic!("Invalid bit")
            }
        }
    }

    let mut gamma_bits: String = String::from("");
    let mut epsilon_bits: String = String::from("");

    for index in indices {
        let counts = counts_by_index[index];
        let most_common_bit: char = if counts.ones > counts.zeros { '1' } else { '0' };
        let least_common_bit: char = if most_common_bit == '1' { '0' } else { '1' };
        gamma_bits.push(most_common_bit);
        epsilon_bits.push(least_common_bit);
    }

    let gamma = u32::from_str_radix(gamma_bits.as_str(), 2).unwrap();
    let epsilon = u32::from_str_radix(epsilon_bits.as_str(), 2).unwrap();

    let solution = gamma * epsilon;

    println!("{}", solution);
}

pub fn part_2() {
    let input_lines = file_helpers::read_file_as_lines("day_03/input.txt");

    let oxygen_rating_binary = get_oxygen_rating(&input_lines, 0);
    let oxygen_rating = get_decimal_from_binary_string(&oxygen_rating_binary);
    println!("o2: {}; {}", oxygen_rating, oxygen_rating_binary);

    let co2_rating_binary = get_co2_rating(&input_lines, 0);
    let co2_rating = get_decimal_from_binary_string(&co2_rating_binary);
    println!("co2: {}; {}", co2_rating, co2_rating_binary);

    let solution = oxygen_rating * co2_rating;
    println!("{}", solution);
}

fn get_oxygen_rating(input_lines: &Vec<String>, bit_index: usize) -> String {
    let matches = filter_oxygen(input_lines, bit_index);

    return if matches.len() > 1 {
        get_oxygen_rating(&matches, bit_index + 1)
    } else {
        matches.first().unwrap().clone()
    }
}

fn get_co2_rating(input_lines: &Vec<String>, bit_index: usize) -> String {
    let matches = filter_co2(input_lines, bit_index);

    return if matches.len() > 1 {
        get_co2_rating(&matches, bit_index + 1)
    } else {
        matches.first().unwrap().clone()
    }
}

fn filter_oxygen(input_lines: &Vec<String>, bit_index: usize) -> Vec<String> {
    let bit_counts = count_bit_at_index(input_lines, bit_index);
    let most_significant_bit = if bit_counts.ones >= bit_counts.zeros { '1' } else { '0' };
    let filtered = filter_by_bit_at_index(&input_lines, most_significant_bit, bit_index);
    return filtered;
}

fn filter_co2(input_lines: &Vec<String>, bit_index: usize) -> Vec<String> {
    let bit_counts = count_bit_at_index(input_lines, bit_index);
    let least_significant_bit = if bit_counts.zeros <= bit_counts.ones { '0' } else { '1' };
    let filtered = filter_by_bit_at_index(&input_lines, least_significant_bit, bit_index);
    return filtered;
}

fn count_bit_at_index(input_lines: &Vec<String>, index: usize) -> BitCounts {
    let mut counts = BitCounts { ones: 0, zeros: 0 };

    for line in input_lines {
        let bit_char = line.chars().nth(index).unwrap();

        if bit_char == '1' {
            counts.ones = counts.ones + 1;
        } else if bit_char == '0' {
            counts.zeros = counts.zeros + 1;
        } else {
            panic!("Invalid bit")
        }
    }

    // println!("There are {} ones and {} zeros at position {}", counts.ones, counts.zeros, index);

    return counts;
}

fn filter_by_bit_at_index(input_lines: &Vec<String>, bit_char: char, index: usize) -> Vec<String> {
    let mut filtered: Vec<String> = Vec::new();

    for line in input_lines {
        if line.chars().nth(index).unwrap() == bit_char {
            filtered.push(line.clone());
        }
    }

    return filtered;
}

fn get_decimal_from_binary_string(binary_string: &String) -> u32 {
    return u32::from_str_radix(binary_string.clone().as_str(), 2).unwrap()
}
