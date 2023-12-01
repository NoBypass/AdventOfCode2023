#[cfg(test)]
mod tests {
    use crate::trebuchet;

    #[test]
    fn test_trebuchet_simple() {
        assert_eq!(trebuchet::run("src/trebuchet/inputs/1.txt"), 142);
    }

    #[test]
    fn test_trebuchet_given() {
        assert_eq!(trebuchet::run("src/trebuchet/inputs/2.txt"), 55488);
    }
}
