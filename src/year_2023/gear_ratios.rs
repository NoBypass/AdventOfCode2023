use crate::helpers::Point;
use crate::year_2023::Solution;

pub struct GearRatios;


impl Solution for GearRatios {
    fn part_one(input: &str) -> i32 {
        for (y, line) in input.lines().enumerate() {}

        0
    }

    fn part_two(input: &str) -> i32 {
        todo!()
    }
}

fn get_numbers(line: &str, y: i32) -> Vec<Number> {
    todo!()
}

struct Number {
    value: i32,
    points: Vec<Point>,
}

struct Symbol {
    covering: [Point; 9],
}

mod tests {
    use crate::year_2023::gear_ratios::GearRatios;
    use crate::year_2023::Solution;

    #[test]
    fn test_part_1_simple() {
        assert_eq!(GearRatios::part_one("467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."), 4361);
    }

    #[test]
    fn test_part_1_long() {}

    #[test]
    fn test_part_2_simple() {}

    #[test]
    fn test_part_2_long() {}
}
