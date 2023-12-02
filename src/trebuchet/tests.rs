#[cfg(test)]
mod tests {
    use crate::trebuchet;

    #[test]
    fn test_trebuchet_simple() {
        assert_eq!(trebuchet::part_one("src/trebuchet/inputs/1.txt"), 142);
    }

    #[test]
    fn test_trebuchet_long() {
        assert_eq!(trebuchet::part_one("src/trebuchet/inputs/2.txt"), 55488);
    }

    #[test]
    fn test_trebuchet_words_simple() {
        assert_eq!(trebuchet::part_two("src/trebuchet/inputs/3.txt"), 281);
    }

    #[test]
    fn test_trebuchet_words_long() {
        assert_eq!(trebuchet::part_two("src/trebuchet/inputs/2.txt"), 55614);
    }
}
