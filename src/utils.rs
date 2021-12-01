use std::fs;

pub fn read_input_lines_all(filename: &str) -> Vec<String> {
    fs::read_to_string(format!("./res/{}", filename))
        .expect(format!("Could not read input file {}", filename).as_str())
        .trim()
        .split("\n")
        .map(|line| line.to_string())
        .collect::<Vec<String>>()
}
