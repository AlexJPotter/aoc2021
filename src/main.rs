pub mod file_helpers;
pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;
pub mod day_05;
pub mod day_06;
pub mod day_07;
pub mod day_08;
pub mod day_09;
pub mod day_10;
pub mod day_11;
pub mod day_12;
pub mod day_13;

use std::time::Instant;
pub use crate::day_13::solution;

fn main() {
    let now = Instant::now();
    solution::part_2();
    println!("Completed in {}s / {}ms / {}μs", now.elapsed().as_secs(), now.elapsed().as_millis(), now.elapsed().as_micros());
}
