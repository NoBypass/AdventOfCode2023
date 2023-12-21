use regex::Regex;

use crate::year_2023::Solution;

pub struct CubeConundrum;

impl Solution for CubeConundrum {
    fn part_one(input: &str) -> i32 {
        input
            .lines()
            .map(|line| parse_line(line))
            .filter(|game| !game.is_impossible())
            .map(|game| game.id)
            .sum::<i32>()
    }

    fn part_two(input: &str) -> i32 {
        input
            .lines()
            .map(|line| parse_line(line))
            .map(|game| game.get_smallest())
            .sum::<i32>()
    }
}

struct Game {
    id: i32,
    sets: Vec<Set>,
}

struct Set {
    red: i32,
    green: i32,
    blue: i32,
}

impl Game {
    fn is_impossible(&self) -> bool {
        for set in &self.sets {
            if set.red > 12 || set.green > 13 || set.blue > 14 {
                return true;
            }
        }

        return false;
    }

    fn get_smallest(&self) -> i32 {
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;
        for set in &self.sets {
            if min_red < set.red {
                min_red = set.red
            }
            if min_green < set.green {
                min_green = set.green
            }
            if min_blue < set.blue {
                min_blue = set.blue
            }
        }
        min_blue * min_green * min_red
    }
}

fn parse_line(line: &str) -> Game {
    let after_split: Vec<&str> = line.split(": ").collect();
    let id: i32 = after_split[0]
        .replace("Game ", "")
        .parse()
        .unwrap();

    let set_strings: Vec<&str> = after_split[1].split("; ").collect();
    let sets: Vec<Set> = set_strings
        .iter()
        .map(|line| {
            Set {
                red: parse_color("red", &line),
                green: parse_color("green", &line),
                blue: parse_color("blue", &line),
            }
        })
        .collect();


    Game { id, sets }
}

fn parse_color(color: &str, line: &str) -> i32 {
    let red_regex = Regex::new(&*format!(r"(\d+) {}", color)).unwrap();

    let red = match red_regex.captures(line) {
        Some(cap) => cap.get(1).map_or("0", |x| x.as_str()),
        None => "0"
    };

    red.parse::<i32>().unwrap()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::year_2023::cube_conundrum::CubeConundrum;
    use crate::year_2023::Solution;

    #[test]
    fn test_cube_conundrum_1_simple() {
        assert_eq!(CubeConundrum::part_one("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 9999999 blue, 3 green; 2 blue, 1 red, 2 green"), 3);
    }

    #[test]
    fn test_cube_conundrum_1_long() {
        assert_eq!(CubeConundrum::part_one(fs::read_to_string("src/inputs/cube_conundrum").expect("").as_str()), 2369);
    }

    #[test]
    fn test_cube_conundrum_2_simple() {
        assert_eq!(CubeConundrum::part_two("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"), 2286);
    }

    #[test]
    fn test_cube_conundrum_2_long() {
        assert_eq!(CubeConundrum::part_two(fs::read_to_string("src/inputs/cube_conundrum").expect("").as_str()), 66363);
    }
}
