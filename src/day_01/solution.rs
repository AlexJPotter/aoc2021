pub use crate::file_helpers;

pub fn part_1() {
    let input_as_string = file_helpers::read_file_as_string("day_01/input.txt");
    let trimmed_input = input_as_string.trim();
    let split_input = trimmed_input.split("\n");

    let mut optional_previous: Option<u32> = Option::None;
    let mut increment_count: u32 = 0;

    for line in split_input {
        let current: u32 = line.to_string().trim().parse().expect("Could not parse input to a number");

        match optional_previous {
            Some(previous) => {
                if current > previous {
                    increment_count = increment_count + 1;
                }
            }
            _ => {}
        }

        optional_previous = Option::from(current);
    }

    println!("{}", increment_count);
}

pub fn part_2() {
    let input_as_string = file_helpers::read_file_as_string("day_01/input.txt");
    let trimmed_input = input_as_string.trim();
    let split_input = trimmed_input.split("\n");

    let mut sliding_values: [Option<u32>; 3] = [None, None, None];
    let mut increment_count: u32 = 0;

    for line in split_input {
        let current: u32 = line.to_string().trim().parse().expect("Could not parse input to a number");

        let s1 = sliding_values[0];
        let s2 = sliding_values[1];
        let s3 = sliding_values[2];

        let s1_next = s2.clone();
        let s2_next = s3.clone();
        let s3_next = Option::from(current);

        let all_values: [Option<u32>; 6] = [s1, s2, s3, s1_next, s2_next, s3_next];

        if all_values.iter().all(|o| o.is_some()) {
            let previous_sum = s1.unwrap() + s2.unwrap() + s3.unwrap();
            let next_sum = s1_next.unwrap() + s2_next.unwrap() + s3_next.unwrap();

            if previous_sum < next_sum {
                increment_count = increment_count + 1;
            }
        }

        sliding_values[0] = s1_next;
        sliding_values[1] = s2_next;
        sliding_values[2] = s3_next;
    }

    println!("{}", increment_count);
}
