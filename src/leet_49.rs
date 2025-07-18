use std::collections::{HashMap};

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    // TODO: Implement your solution here
    let mut map:HashMap<Vec<char>,Vec<String>> = HashMap::new();
    for str in strs {

        let mut chars: Vec<char> = str.chars().collect();
        chars.sort();

        if let Some(list) = map.get_mut(&chars) {
            list.push(str);
        }else{
            map.insert(chars, vec![str]);
        }

    }

    map.values().cloned().collect::<Vec<Vec<String>>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_case() {
        let strs = vec!["eat".to_string(), "tea".to_string(), "tan".to_string(), "ate".to_string(), "nat".to_string(), "bat".to_string()];
        let mut result = group_anagrams(strs);

        // Sort each group and then sort the groups for comparison
        for group in &mut result {
            group.sort();
        }
        result.sort();

        let mut expected = vec![
            vec!["ate".to_string(), "eat".to_string(), "tea".to_string()],
            vec!["bat".to_string()],
            vec!["nat".to_string(), "tan".to_string()]
        ];
        for group in &mut expected {
            group.sort();
        }
        expected.sort();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_empty_string() {
        let strs = vec!["".to_string()];
        let result = group_anagrams(strs);
        assert_eq!(result, vec![vec!["".to_string()]]);
    }

    #[test]
    fn test_single_character() {
        let strs = vec!["a".to_string()];
        let result = group_anagrams(strs);
        assert_eq!(result, vec![vec!["a".to_string()]]);
    }

    #[test]
    fn test_all_same_anagrams() {
        let strs = vec!["abc".to_string(), "bca".to_string(), "cab".to_string()];
        let mut result = group_anagrams(strs);

        // Should have only one group
        assert_eq!(result.len(), 1);
        result[0].sort();

        let mut expected = vec!["abc".to_string(), "bca".to_string(), "cab".to_string()];
        expected.sort();
        assert_eq!(result[0], expected);
    }

    #[test]
    fn test_no_anagrams() {
        let strs = vec!["abc".to_string(), "def".to_string(), "ghi".to_string()];
        let result = group_anagrams(strs);
        assert_eq!(result.len(), 3); // Each string should be in its own group
    }

    #[test]
    fn test_repeated_strings() {
        let strs = vec!["abc".to_string(), "abc".to_string(), "def".to_string()];
        let mut result = group_anagrams(strs);

        for group in &mut result {
            group.sort();
        }
        result.sort();

        let mut expected = vec![
            vec!["abc".to_string(), "abc".to_string()],
            vec!["def".to_string()]
        ];
        for group in &mut expected {
            group.sort();
        }
        expected.sort();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_mixed_lengths() {
        let strs = vec!["a".to_string(), "aa".to_string(), "aaa".to_string()];
        let result = group_anagrams(strs);
        assert_eq!(result.len(), 3); // Different lengths can't be anagrams
    }

    #[test]
    fn test_case_sensitivity() {
        let strs = vec!["Abc".to_string(), "bca".to_string(), "CaB".to_string()];
        let result = group_anagrams(strs);
        assert_eq!(result.len(), 3); // Case matters
    }

    #[test]
    fn test_empty_vector() {
        let strs = vec![];
        let result = group_anagrams(strs);
        assert_eq!(result, Vec::<Vec<String>>::new());
    }

    #[test]
    fn test_long_strings() {
        let strs = vec![
            "listen".to_string(),
            "silent".to_string(),
            "hello".to_string(),
            "world".to_string(),
            "enlist".to_string()
        ];
        let mut result = group_anagrams(strs);

        for group in &mut result {
            group.sort();
        }
        result.sort();

        // listen, silent, enlist should be grouped together
        assert_eq!(result.len(), 3);

        // Find the anagram group
        let anagram_group = result.iter().find(|group| group.len() == 3).unwrap();
        let mut expected_anagrams = vec!["listen".to_string(), "silent".to_string(), "enlist".to_string()];
        expected_anagrams.sort();
        assert_eq!(anagram_group, &expected_anagrams);
    }
}
