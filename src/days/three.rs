use crate::utils::read_input_lines_all;

/*
https://adventofcode.com/2021/day/3


*/

pub fn run() {
    let lines = read_input_lines_all("three.txt");
    assess_diagnostics(lines);
}

fn assess_diagnostics(data: Vec<String>) -> (i32, i32, i32, i32, i32, i32) {
    println!("Day three");
    println!("What is the power consumption of the submarine?");

    let sums = sum_bits(&data);

    let (gamma_reading, epsilon_reading) =
        sums.iter()
            .fold((String::new(), String::new()), |mut reading, sum| {
                reading.0.push_str(most_common(sum).to_string().as_str());
                reading.1.push_str(least_common(sum).to_string().as_str());
                reading
            });

    let gamma_rate = to_decimal(&gamma_reading);
    let epsilon_rate = to_decimal(&epsilon_reading);
    let power_consumption = gamma_rate * epsilon_rate;

    /*
      part 2
      let's not speak of this again
    */

    let mut relevant_indexes = data.clone();
    let mut bit = 0;
    for i in 0..sums.len() {
        let sum = sum_bits(&relevant_indexes)[i];
        let seek = if sum.1 >= sum.0 { '1' } else { '0' };
        relevant_indexes = relevant_indexes
            .into_iter()
            .filter(|s| s.chars().nth(bit).unwrap() == seek)
            .collect();
        if relevant_indexes.len() == 1 {
            break;
        }
        bit += 1;
    }

    let ovalue = relevant_indexes.get(0).unwrap();
    let oxygen_rating = to_decimal(ovalue);

    let mut relevant_indexes = data.clone();
    let mut bit = 0;
    for i in 0..sums.len() {
        let sum = sum_bits(&relevant_indexes)[i];
        let seek = if sum.0 <= sum.1 { '0' } else { '1' };
        relevant_indexes = relevant_indexes
            .into_iter()
            .filter(|s| s.chars().nth(bit).unwrap() == seek)
            .collect();
        if relevant_indexes.len() == 1 {
            break;
        }
        bit += 1;
    }

    let co2value = relevant_indexes.get(0).unwrap();
    let co2_rating = to_decimal(co2value);

    let life_support_rating = co2_rating * oxygen_rating;

    println!("The gamma rate is {}", gamma_rate.to_string());
    println!("The epsilon rate is {}", epsilon_rate);
    println!("The power consumption is {}", power_consumption.to_string());
    println!("-----");
    println!("The oxygen generator rating is {}", oxygen_rating);
    println!("The CO2 scrubber rating is {}", co2_rating);
    println!("The life support rating is {}", life_support_rating);

    (
        gamma_rate,
        epsilon_rate,
        power_consumption,
        oxygen_rating,
        co2_rating,
        life_support_rating,
    )
}

fn sum_bits(data: &Vec<String>) -> Vec<(i32, i32)> {
    data.iter().fold(vec![], |mut acc, line| {
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
    })
}

fn to_decimal(input: &String) -> i32 {
    i32::from_str_radix(input, 2).unwrap()
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
            (22, 9, 198, 23, 10, 230)
        );
    }
}
