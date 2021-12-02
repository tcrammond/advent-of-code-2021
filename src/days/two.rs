use crate::utils::read_input_lines_all;

/*
https://adventofcode.com/2021/day/2


*/

pub fn run() {
    let lines = read_input_lines_all("two.txt");
    assess_commands(lines);
}

#[derive(Debug)]
enum SubmarineCommand {
    Horizontal(i32),
    Vertical(i32),
}

fn assess_commands(data: Vec<String>) -> i32 {
    println!("Day two");
    println!("What is the product of the commands?");

    let commands: Vec<Result<SubmarineCommand, &str>> =
        data.iter().map(|line| parse_command(line)).collect();

    if commands.iter().any(|command| command.is_err()) {
        println!("Command list not valid");
        return -1;
    }

    let mut horizontal_progress = 0;
    let mut vertical_progress = 0;

    for c in commands {
        if let Ok(command) = c {
            match command {
                SubmarineCommand::Horizontal(value) => {
                    horizontal_progress += value;
                }
                SubmarineCommand::Vertical(value) => {
                    vertical_progress += value;
                }
            }
        }
    }

    let product = horizontal_progress * vertical_progress;
    println!(
        "The horizontal progress is {}",
        horizontal_progress.to_string()
    );
    println!("The vertical progress is {}", vertical_progress);
    println!("The answer is {}", product.to_string());

    product
}

fn parse_command(input: &str) -> Result<SubmarineCommand, &str> {
    let parts = input.split_once(" ");
    if let None = parts {
        return Err("Invalid command");
    }
    let parts = parts.unwrap();
    let value = parts.1.parse::<i32>();

    if let Err(_) = value {
        return Err("Invalid command");
    }

    match parts.0 {
        "forward" => Ok(SubmarineCommand::Horizontal(value.unwrap())),
        "up" => Ok(SubmarineCommand::Vertical(-value.unwrap())),
        "down" => Ok(SubmarineCommand::Vertical(value.unwrap())),
        _ => Err("Unknown command"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_with_example_data() {
        assert_eq!(
            assess_commands(vec![
                "forward 5".to_string(),
                "down 5".to_string(),
                "forward 8".to_string(),
                "up 3".to_string(),
                "down 8".to_string(),
                "forward 2".to_string(),
            ]),
            150
        );
    }
}
