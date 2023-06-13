// Simplest form to work with integers
pub fn linear_search_ints(arr: &[i32], target: &i32) -> Option<usize> {}

// Another version to search chars
pub fn linear_search_chars(arr: &[char], target: &char) -> Option<usize> {}

// Another version to search strings
pub fn linear_search_strings(arr: &[String], target: &str) -> Option<usize> {}

// Implementing a generic version
pub fn linear_search<T: PartialEq>(arr: &[T], target: &T) -> Option<usize> {}

#[cfg(test)]
mod test {
    use super::linear_search_chars;
    use super::linear_search_ints;
    use super::linear_search_strings;

    #[test]
    fn search_ints() {
        assert_eq!(linear_search_ints(&[1, 2, 3, 4], &1), Some(0));
        assert_eq!(linear_search_ints(&[1, 2, 3, 4], &2), Some(1));
        assert_eq!(linear_search_ints(&[1, 2, 3, 4], &3), Some(2));
        assert_eq!(linear_search_ints(&[1, 2, 3, 4], &4), Some(3));
        assert_eq!(linear_search_ints(&[1, 2, 3, 4], &5), None);
    }

    #[test]
    fn search_chars() {
        assert_eq!(linear_search_chars(&['a', 'b', 'c', 'd'], &'a'), Some(0));
        assert_eq!(linear_search_chars(&['a', 'b', 'c', 'd'], &'b'), Some(1));
        assert_eq!(linear_search_chars(&['a', 'b', 'c', 'd'], &'c'), Some(2));
        assert_eq!(linear_search_chars(&['a', 'b', 'c', 'd'], &'d'), Some(3));
        assert_eq!(linear_search_chars(&['a', 'b', 'c', 'd'], &'e'), None);
    }

    #[test]
    fn search_strings() {
        assert_eq!(
            linear_search_strings(
                &["Rust".to_string(), "is".to_string(), "Great".to_string()],
                &"Rust"
            ),
            Some(0)
        );
        assert_eq!(
            linear_search_strings(
                &["Rust".to_string(), "is".to_string(), "Great".to_string()],
                &"is"
            ),
            Some(1)
        );
        assert_eq!(
            linear_search_strings(
                &["Rust".to_string(), "is".to_string(), "Great".to_string()],
                &"Great"
            ),
            Some(2)
        );
        assert_eq!(
            linear_search_strings(
                &["Rust".to_string(), "is".to_string(), "Great".to_string()],
                &"Not"
            ),
            None
        );
    }
}
