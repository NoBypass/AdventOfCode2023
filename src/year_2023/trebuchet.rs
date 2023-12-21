use crate::year_2023::Solution;

pub struct Trebuchet;

impl Solution for Trebuchet {
    fn part_one(contents: &str) -> i32 {
        contents
            .lines()
            .map(|line| {
                let mut it = line.chars().filter_map(|char| char.to_digit(10));
                let first = it.next().expect("Should be a number");
                match it.last() {
                    Some(num) => format!("{first}{num}"),
                    None => format!("{first}{first}"),
                }
                    .parse::<i32>()
                    .expect("Should be a valid number")
            })
            .sum::<i32>()
    }

    fn part_two(contents: &str) -> i32 {
        contents
            .lines()
            .map(|line| {
                let mut it = (0..line.len()).filter_map(|index| {
                    let reduced_line = &line[index..];
                    let result = if reduced_line.starts_with("one") {
                        '1'
                    } else if reduced_line.starts_with("two") {
                        '2'
                    } else if reduced_line.starts_with("three") {
                        '3'
                    } else if reduced_line.starts_with("four") {
                        '4'
                    } else if reduced_line.starts_with("five") {
                        '5'
                    } else if reduced_line.starts_with("six") {
                        '6'
                    } else if reduced_line.starts_with("seven") {
                        '7'
                    } else if reduced_line.starts_with("eight") {
                        '8'
                    } else if reduced_line.starts_with("nine") {
                        '9'
                    } else {
                        reduced_line.chars().next().unwrap()
                    };

                    result.to_digit(10)
                });
                let first = it.next().expect("should be a number");
                match it.last() {
                    Some(num) => format!("{first}{num}"),
                    None => format!("{first}{first}"),
                }
                    .parse::<i32>()
                    .expect("should be a valid number")
            })
            .sum::<i32>()
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::year_2023::Solution;
    use crate::year_2023::trebuchet::Trebuchet;

    #[test]
    fn test_trebuchet_1_simple() {
        assert_eq!(Trebuchet::part_one("1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"), 142);
    }

    #[test]
    fn test_trebuchet_1_long() {
        assert_eq!(Trebuchet::part_one(fs::read_to_string("src/inputs/trebuchet").expect("").as_str()), 55488);
    }

    #[test]
    fn test_trebuchet_2_simple() {
        assert_eq!(Trebuchet::part_two("two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"), 281);
    }

    #[test]
    fn test_trebuchet_2_long() {
        assert_eq!(Trebuchet::part_two(fs::read_to_string("src/inputs/trebuchet").expect("").as_str()), 55614);
    }
}
