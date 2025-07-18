use std::collections::HashMap;

pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
    // TODO: Implement your solution here
    
    let p_len = p.len();
    if s.len() < p_len {
        return Vec::new();
    }
    let s_chars = s.chars().collect::<Vec<char>>();
    
    let mut p_table = HashMap::new();
    for char in p.chars() {
        *p_table.entry(char).or_insert(0) +=1;
    }

    let mut window =  HashMap::new();
    let mut ret = Vec::new();
    
    for i in 0..s_chars.len(){
        let s_char = s_chars[i];
        *window.entry(s_char).or_insert(0) += 1;
        
        if i >= p_len{
            let index = i - p_len;
            let remove_char = s_chars[index];
            
            if let Some(count) = window.get_mut(&remove_char){
                *count -=1;
                if *count == 0{
                    window.remove(&remove_char);
                }
            }
        }

        if i >= p_len - 1 && window == p_table {
            ret.push((i - p_len + 1 ) as i32);
        }
    }
    ret
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_case() {
        let s = "abab".to_string();
        let p = "ab".to_string();
        let result = find_anagrams(s, p);
        assert_eq!(result, vec![0, 1, 2]);
    }

    #[test]
    fn test_no_anagrams() {
        let s = "abcd".to_string();
        let p = "xyz".to_string();
        let result = find_anagrams(s, p);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_same_length_strings() {
        let s = "abc".to_string();
        let p = "bca".to_string();
        let result = find_anagrams(s, p);
        assert_eq!(result, vec![0]);
    }

    #[test]
    fn test_same_length_no_match() {
        let s = "abc".to_string();
        let p = "def".to_string();
        let result = find_anagrams(s, p);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_p_longer_than_s() {
        let s = "ab".to_string();
        let p = "abc".to_string();
        let result = find_anagrams(s, p);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_empty_s() {
        let s = "".to_string();
        let p = "a".to_string();
        let result = find_anagrams(s, p);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_single_character_match() {
        let s = "aaa".to_string();
        let p = "a".to_string();
        let result = find_anagrams(s, p);
        assert_eq!(result, vec![0, 1, 2]);
    }

    #[test]
    fn test_single_character_no_match() {
        let s = "bbb".to_string();
        let p = "a".to_string();
        let result = find_anagrams(s, p);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_all_same_characters() {
        let s = "aaaa".to_string();
        let p = "aa".to_string();
        let result = find_anagrams(s, p);
        assert_eq!(result, vec![0, 1, 2]);
    }
    
    #[test]
    fn test_long_pattern() {
        let s = "abcdefghijklmnop".to_string();
        let p = "def".to_string();
        let result = find_anagrams(s, p);
        assert_eq!(result, vec![3]);
    }
    

    #[test]
    fn test_performance_case() {
        let s = "a".repeat(1000);
        let p = "aa".to_string();
        let result = find_anagrams(s, p);
        assert_eq!(result.len(), 999);
        assert_eq!(result[0], 0);
        assert_eq!(result[998], 998);
    }

    #[test]
    fn test_mixed_characters() {
        let s = "bpaa".to_string();
        let p = "aa".to_string();
        let result = find_anagrams(s, p);
        assert_eq!(result, vec![2]);
    }

    #[test]
    fn test_palindromic_anagrams() {
        let s = "abba".to_string();
        let p = "ab".to_string();
        let result = find_anagrams(s, p);
        assert_eq!(result, vec![0, 2]);
    }
}