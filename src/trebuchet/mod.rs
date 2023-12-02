mod tests;

use std::fs;

pub fn part_one(path: &str) -> i32 {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

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

pub fn part_two(path: &str) -> i32 {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

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
