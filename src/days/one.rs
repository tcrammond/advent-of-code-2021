use crate::utils::read_input_lines_all;

pub fn run() {
    println!("Day one");
    println!("How many times does the depth increase?");

    let lines = read_input_lines_all("one.txt")
        .iter()
        .map(|value| value.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut increased_count = 0;
    let mut prev_value: Option<i32> = None;

    for value in lines {
        if prev_value.is_some() {
            if value > prev_value.unwrap() {
                increased_count += 1;
            }
        }

        prev_value = Some(value);
    }

    println!("The depth inrceased {} times", increased_count.to_string());
}
