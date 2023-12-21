pub fn max(max: usize, actual: usize) -> usize {
    if actual > max {
        max
    }
    actual
}

pub fn min(min: usize, actual: usize) -> usize {
    if actual < min {
        min
    }
    actual
}

pub struct Point {
    pub x: i32,
    pub y: i32,
}
