use crate::utils::read_input_lines_all;

/*
https://adventofcode.com/2021/day/1

count the number of times the sum of measurements in this sliding window increases from the previous sum
*/

pub fn run() {
    let lines: Vec<i32> = read_input_lines_all("one.txt")
        .iter()
        .map(|value| value.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    assess_depth(lines);
}

fn assess_depth(data: Vec<i32>) -> i32 {
    println!("Day one");
    println!("How many times does the depth increase?");

    let mut increased_count = 0;
    let mut prev_value: Option<i32> = None;

    data.iter().fold(vec![], |acc, value| {
        let mut next = acc.clone();
        next.push(value.to_owned());
        if next.len() < 3 {
            return next;
        }

        let sum = next.iter().sum::<i32>();
        if prev_value.is_some() && sum > prev_value.unwrap() {
            increased_count += 1;
        }
        prev_value = Some(sum);
        next.remove(0);

        next
    });

    println!("The depth inrceased {} times", increased_count.to_string());
    increased_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_with_example_data() {
        assert_eq!(
            assess_depth(vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263,]),
            5
        );
    }
}
