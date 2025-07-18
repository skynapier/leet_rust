pub fn count_substrings(s: String) -> i32 {
    // TODO: Implement your solution here
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_case() {
        let s = "abc".to_string();
        let result = count_substrings(s);
        assert_eq!(result, 3); // "a", "b", "c"
    }

    #[test]
    fn test_repeated_characters() {
        let s = "aaa".to_string();
        let result = count_substrings(s);
        assert_eq!(result, 6); // "a", "a", "a", "aa", "aa", "aaa"
    }

    #[test]
    fn test_mixed_palindromes() {
        let s = "aba".to_string();
        let result = count_substrings(s);
        assert_eq!(result, 4); // "a", "b", "a", "aba"
    }

    #[test]
    fn test_single_character() {
        let s = "a".to_string();
        let result = count_substrings(s);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_empty_string() {
        let s = "".to_string();
        let result = count_substrings(s);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_no_palindromes_except_singles() {
        let s = "abcd".to_string();
        let result = count_substrings(s);
        assert_eq!(result, 4); // Only single character palindromes
    }

    #[test]
    fn test_all_same_characters() {
        let s = "aaaa".to_string();
        let result = count_substrings(s);
        assert_eq!(result, 10); // 4 + 3 + 2 + 1 = 10
    }

    #[test]
    fn test_even_length_palindromes() {
        let s = "abba".to_string();
        let result = count_substrings(s);
        assert_eq!(result, 6); // "a", "b", "b", "a", "bb", "abba"
    }

    #[test]
    fn test_mixed_even_odd_palindromes() {
        let s = "abccba".to_string();
        let result = count_substrings(s);
        assert_eq!(result, 9); // "a", "b", "c", "c", "b", "a", "cc", "bccb", "abccba"
    }

    #[test]
    fn test_long_palindrome() {
        let s = "racecar".to_string();
        let result = count_substrings(s);
        assert_eq!(result, 10); // Single chars + some palindromes
    }

    #[test]
    fn test_nested_palindromes() {
        let s = "abcba".to_string();
        let result = count_substrings(s);
        assert_eq!(result, 7); // "a", "b", "c", "b", "a", "bcb", "abcba"
    }

    #[test]
    fn test_alternating_pattern() {
        let s = "ababa".to_string();
        let result = count_substrings(s);
        assert_eq!(result, 9); // "a", "b", "a", "b", "a", "aba", "bab", "aba", "ababa"
    }

    #[test]
    fn test_case_sensitivity() {
        let s = "Aa".to_string();
        let result = count_substrings(s);
        assert_eq!(result, 2); // "A", "a" (case sensitive)
    }

    #[test]
    fn test_complex_palindromes() {
        let s = "aabaa".to_string();
        let result = count_substrings(s);
        assert_eq!(result, 9); // "a", "a", "b", "a", "a", "aa", "aa", "aba", "aabaa"
    }

    #[test]
    fn test_overlapping_palindromes() {
        let s = "aaaba".to_string();
        let result = count_substrings(s);
        assert_eq!(result, 8); // "a", "a", "a", "b", "a", "aa", "aa", "aaa"
    }

    #[test]
    fn test_double_character_palindromes() {
        let s = "aabbcc".to_string();
        let result = count_substrings(s);
        assert_eq!(result, 9); // 6 single chars + 3 double chars
    }

    #[test]
    fn test_mirror_pattern() {
        let s = "abcdef".to_string();
        let result = count_substrings(s);
        assert_eq!(result, 6); // Only single character palindromes
    }

    #[test]
    fn test_performance_case() {
        let s = "a".repeat(100);
        let result = count_substrings(s);
        assert_eq!(result, 5050); // n*(n+1)/2 where n=100
    }

    #[test]
    fn test_edge_case_two_chars() {
        let s = "ab".to_string();
        let result = count_substrings(s);
        assert_eq!(result, 2); // "a", "b"
    }

    #[test]
    fn test_edge_case_two_same_chars() {
        let s = "aa".to_string();
        let result = count_substrings(s);
        assert_eq!(result, 3); // "a", "a", "aa"
    }
}