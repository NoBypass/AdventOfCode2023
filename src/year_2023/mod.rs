mod cube_conundrum;
mod trebuchet;
mod gear_ratios;

pub trait Solution {
    fn part_one(input: &str) -> i32;
    fn part_two(input: &str) -> i32;
}
