use std::fs;

pub fn read_file_as_string(relative_path: &str) -> String {
    let root_directory = "C:/workspace/aoc/aoc2021/src/";
    let absolute_path = root_directory.to_string() + relative_path;
    return fs::read_to_string(absolute_path).expect("Could not read file");
}

pub fn read_file_as_lines(relative_path: &str) -> Vec<String> {
    let file_as_string = read_file_as_string(relative_path);
    let normalised = file_as_string.replace("\r\n", "\n");
    let split = normalised.split("\n");

    let mut vector: Vec<String> = Vec::new();

    for line in split {
        let string_line = String::from(line.trim());

        if !string_line.is_empty() {
            vector.push(string_line);
        }
    }

    return vector;
}

pub fn read_file_as_raw_lines(relative_path: &str) -> Vec<String> {
    let file_as_string = read_file_as_string(relative_path);
    let normalised = file_as_string.replace("\r\n", "\n");
    let split = normalised.split("\n");

    let mut vector: Vec<String> = Vec::new();

    for line in split {
        let string_line = String::from(line);
        vector.push(string_line);
    }

    return vector;
}
