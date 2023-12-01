mod tests;

use std::fs;

pub fn run(path: &str) -> i32 {
    let contents = fs::read_to_string(path)
        .expect("Something went wrong reading the file");

    let lines: Vec<i32> = contents.lines().map(|line| parse_line(line)).collect();
    return lines.iter().sum();
}

fn parse_line(line: &str) -> i32 {
    let parts: Vec<&str> = line.split("").collect();
    let nums: Vec<i32> = parts.iter().map(|char| char.parse::<i32>().unwrap_or(0)).collect();
    let mut first = 0;
    let mut last = 0;
    nums.iter().for_each(|num| {
        if first == 0 && *num != 0 {
            first = *num;
        }
        if *num != 0 {
            last = *num;
        }
    });
    return format!("{}{}", first, last).parse::<i32>().unwrap_or(0);
}