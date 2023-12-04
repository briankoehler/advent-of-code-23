use std::{path::Path, collections::HashMap};
use crate::{Solution, utils::read_input_to_string_vec};

pub struct Day2 {
    input: Vec<String>,
}

impl Solution for Day2 {
    fn new(filename: impl AsRef<Path>) -> Self {
        let input = read_input_to_string_vec(filename);
        Day2 { input }
    }

    fn part_1(self) -> String {
        let cubes = HashMap::from([
            ("red", 12),
            ("green", 13),
            ("blue", 14),
        ]);

        let mut sum = 0;
        for line in self.input {
            let colon_index = line.find(':').expect("Failed to find ':' in the line");
            let game_id = &line[5..colon_index];

            let game_sets_raw = &line[colon_index+1..];
            let game_sets = game_sets_raw.split("; ");

            let mut is_ok = true;
            'game_set: for set in game_sets {
                let counts = set.split(", ");
                for count in counts {
                    let mut tokens = count.split_whitespace();
                    let number = tokens.next().unwrap().parse::<u32>().unwrap();

                    if cubes.get(tokens.last().unwrap()).unwrap() < &number {
                        is_ok = false;
                        break 'game_set;
                    }
                }
            }

            if is_ok {
                sum += game_id.parse::<u32>().unwrap();
            }
        }

        sum.to_string()
    }

    fn part_2(self) -> String {
        let mut sum = 0;
        for line in self.input {
            let colon_index = line.find(':').expect("Failed to find ':' in the line");
            let game_sets = line[colon_index+1..].split("; ");

            let mut min_cubes = HashMap::from([
                ("red", 0),
                ("green", 0),
                ("blue", 0),
            ]);

            for set in game_sets {
                let counts = set.split(", ");
                for count in counts {
                    let mut tokens = count.split_whitespace();
                    let number = tokens.next().unwrap().parse::<u32>().unwrap();
                    let color = tokens.next().unwrap();

                    if number > min_cubes[color] {
                        min_cubes.insert(color, number);
                    }
                }
            }

            let power = min_cubes.values().fold(1, |power, e| power * e);
            sum += power;
        }


        sum.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn part_1() {
        let solution = Day2 {
            input: DATA.lines().map(|l| l.to_string()).collect(),
        };
        let calculated = solution.part_1();
        assert_eq!(calculated, String::from("8"));
    }

    #[test]
    fn part_2() {
        let solution = Day2 {
            input: DATA.lines().map(|l| l.to_string()).collect(),
        };
        let calculated = solution.part_2();
        assert_eq!(calculated, String::from("2286"));
    }
}

