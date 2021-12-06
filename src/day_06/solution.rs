use std::time::Instant;
use crate::file_helpers;

pub fn part_1() {
    solve(80);
}

pub fn part_2() {
    solve(256);
}

fn solve(days_to_run: i32) {
    let now = Instant::now();

    let mut counts: [u64; 9] = [0; 9];

    file_helpers::read_file_as_string("day_06/input.txt")
        .trim()
        .split(",")
        .filter(|&s| s.trim() != "")
        .map(|s| to_index(s))
        .for_each(|n| counts[n] = counts[n] + 1);

    let mut days = days_to_run;

    while days > 0 {
        let old_counts = counts.clone();

        counts[8] = old_counts[0];
        counts[7] = old_counts[8];
        counts[6] = old_counts[7] + old_counts[0];
        counts[5] = old_counts[6];
        counts[4] = old_counts[5];
        counts[3] = old_counts[4];
        counts[2] = old_counts[3];
        counts[1] = old_counts[2];
        counts[0] = old_counts[1];

        days = days - 1;
    }

    let solution: u64 = counts.iter().sum();

    println!("{}", solution);
    println!("Completed in 0.{}ms", now.elapsed().as_micros());
}

fn to_index(my_str: &str) -> usize {
    return my_str.parse().expect("Invalid number");
}
