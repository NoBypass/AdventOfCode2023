use std::fs;

pub fn run() -> i32 {
    let contents = fs::read_to_string("src/inputs/trebuchet.txt")
        .expect("Something went wrong reading the file");

    let lines: Vec<i32> = contents.lines().map(|line| parse_line(line)).collect();
    return lines.iter().sum();
}

fn parse_line(line: &str) -> i32 {
    let parts: Vec<&str> = line.split("").collect();
    let nums: Vec<Option<i32>> = parts.iter().map(|char| {
        let num = char.parse::<i32>().unwrap_or(0);
        if num > 0 {
            Some(num)
        } else {
            None
        }
    }).collect();
    return 0;
}
