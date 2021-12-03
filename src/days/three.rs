use crate::utils::read_input_lines_all;

/*
https://adventofcode.com/2021/day/3


*/

pub fn run() {
    let lines = read_input_lines_all("three.txt");
    assess_diagnostics(lines);
}

fn assess_diagnostics(data: Vec<String>) -> (i32, i32, i32) {
    println!("Day three");
    println!("What is the power consumption of the submarine?");

    // Build list of the sums of each bit
    let sums: Vec<(i32, i32)> = data.iter().fold(vec![], |mut acc, line| {
        for (i, v) in line.chars().enumerate() {
            if let None = acc.get(i) {
                acc.push((0, 0));
            }
            if v == '0' {
                acc[i].0 += 1;
            } else {
                acc[i].1 += 1;
            }
        }
        acc
    });

    let mut gamma_reading = String::new();
    sums.iter()
        .for_each(|sum| gamma_reading.push_str(most_common(sum).to_string().as_str()));

    let mut epsilon_reading = String::new();
    sums.iter()
        .for_each(|sum| epsilon_reading.push_str(least_common(sum).to_string().as_str()));

    let gamma_rate = i32::from_str_radix(gamma_reading.as_str(), 2).unwrap();
    let epsilon_rate = i32::from_str_radix(epsilon_reading.as_str(), 2).unwrap();
    let power_consumption = gamma_rate * epsilon_rate;

    println!("The gamma rate is {}", gamma_rate.to_string());
    println!("The epsilon rate is {}", epsilon_rate);
    println!("The power consumption is {}", power_consumption.to_string());

    (gamma_rate, epsilon_rate, power_consumption)
}

fn most_common(input: &(i32, i32)) -> i32 {
    return if input.0 > input.1 { 0 } else { 1 };
}
fn least_common(input: &(i32, i32)) -> i32 {
    return if input.0 > input.1 { 1 } else { 0 };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_with_example_data() {
        assert_eq!(
            assess_diagnostics(vec![
                "00100".to_string(),
                "11110".to_string(),
                "10110".to_string(),
                "10111".to_string(),
                "10101".to_string(),
                "01111".to_string(),
                "00111".to_string(),
                "11100".to_string(),
                "10000".to_string(),
                "11001".to_string(),
                "00010".to_string(),
                "01010".to_string(),
            ]),
            (22, 9, 198)
        );
    }
}
