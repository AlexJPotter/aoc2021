use std::fs;

pub fn read_file_as_string(relative_path: &str) -> String {
    let root_directory = "C:/workspace/aoc/aoc2021/src/";
    let absolute_path = root_directory.to_string() + relative_path;
    return fs::read_to_string(absolute_path).expect("Could not read file");
}
