use std::path::Path;
use std::collections::HashMap;
use crate::{Solution, utils::read_input_to_string_vec};

pub struct Day1 {
    pub input: Vec<String>,
}

impl Solution for Day1 {
    fn new(filename: impl AsRef<Path>) -> Self {
        let input = read_input_to_string_vec(filename);
        Day1 {
            input
        }
    }

    fn part_1(self) -> String {
        let mut sum = 0;
        for line in self.input {
            if line.is_empty() { continue };
            let mut cal_value = String::new();
            
            // Front to back until num
            for c in line.chars() {
                if c.is_numeric() {
                    cal_value.push(c);
                    break;
                }
            }
            
            // Back to front until num
            for c in line.chars().rev() {
                if c.is_numeric() {
                    cal_value.push(c);
                    break;
                }
            }

            sum += cal_value.parse::<u32>().expect("Failed to parse calibration value");
        }

        sum.to_string()
    }

    fn part_2(self) -> String {
        let digits_map = HashMap::from([
            ("one", '1'),
            ("two", '2'),
            ("three", '3'),
            ("four", '4'),
            ("five", '5'),
            ("six", '6'),
            ("seven", '7'),
            ("eight", '8'),
            ("nine", '9'),
        ]);

        let mut sum = 0;
        for line in self.input {
            if line.is_empty() { continue };
            let mut cal_value = String::new();

            // Front to back until num
            'front: for i in 1..line.len()+1 {
                let last_char = line[..i].chars().last().unwrap();
                if last_char.is_numeric() {
                    cal_value.push(last_char);
                    break;
                }

                for digit in digits_map.keys() {
                    if line[..i].ends_with(digit) {
                        cal_value.push(*digits_map.get(digit).unwrap());
                        break 'front;
                    }
                }
            }

            // Back to front until num
            'back: for i in (0..line.len()).rev() {
                let first_char = line[i..].chars().next().unwrap();
                if first_char.is_numeric() {
                    cal_value.push(first_char);
                    break;
                }

                for digit in digits_map.keys() {
                    if line[i..].starts_with(digit) {
                        cal_value.push(*digits_map.get(digit).unwrap());
                        break 'back;
                    }
                }
            }

            sum += cal_value.parse::<u32>().expect("Failed to parse calibration value");
        }

        sum.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
    const TEST_DATA_2: &str = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";

    #[test]
    fn part_1() {
        let solution = Day1 {
            input: TEST_DATA.lines().map(|line| line.to_string()).collect(),
        };

        let calculated = solution.part_1();
        assert_eq!(calculated, String::from("142"));
    }

    #[test]
    fn part_2() {
        let solution = Day1 {
            input: TEST_DATA_2.lines().map(|line| line.to_string()).collect(),
        };

        let calculated = solution.part_2();
        assert_eq!(calculated, String::from("281"));
    }
}

