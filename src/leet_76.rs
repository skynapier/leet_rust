use std::collections::HashMap;

pub fn min_window(s: String, t: String) -> String {
    // TODO: Implement your solution here
    let s_char = s.chars().collect::<Vec<char>>();
    let t_map = convert_to_map(&t);

    let mut window = HashMap::new();
    let len = s.len();
    let (mut left, mut right) = (0, 0);
    let mut need = 0;
    let mut min_len =  usize::MAX;
    let mut start = 0;
    
    while right < len{
        let char = s_char[right];
        right += 1;
        
        
        if t_map.contains_key(&char){
            match window.get_mut(&char) {
                Some(count) => *count += 1,
                None => { window.insert(char, 1); }
            }
            
            if window.get(&char) == t_map.get(&char){
                need+=1;
            }
        }
        
        while need == t_map.len(){
            if (right - left) < min_len{
                start = left;
                min_len = right - left;
            }
            let char = s_char[left];
            left = left + 1;
            
            if t_map.contains_key(&char){
                if window.get(&char) == t_map.get(&char){
                    need -= 1;
                }
                if let Some(count) = window.get_mut(&char) {
                    *count -= 1;
                }
            }
            
        }
    }

    if min_len == usize::MAX {
        String::new()
    } else {
        s_char[start..start + min_len].iter().collect()
    }
}

fn convert_to_map(str:&str) -> HashMap<char,i32>{
    let mut s_map = HashMap::new();
    for char in str.chars(){
        match s_map.get_mut(&char) {
            Some(count) => *count += 1,
            None => { s_map.insert(char, 1); }
        }
    }
    s_map
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_case() {
        let s = "ADOBECODEBANC".to_string();
        let t = "ABC".to_string();
        let result = min_window(s, t);
        assert_eq!(result, "BANC".to_string());
    }

    #[test]
    fn test_no_window() {
        let s = "a".to_string();
        let t = "aa".to_string();
        let result = min_window(s, t);
        assert_eq!(result, "".to_string());
    }

    #[test]
    fn test_exact_match() {
        let s = "a".to_string();
        let t = "a".to_string();
        let result = min_window(s, t);
        assert_eq!(result, "a".to_string());
    }

    #[test]
    fn test_empty_strings() {
        let s = "".to_string();
        let t = "".to_string();
        let result = min_window(s, t);
        assert_eq!(result, "".to_string());
    }

    #[test]
    fn test_t_longer_than_s() {
        let s = "abc".to_string();
        let t = "abcd".to_string();
        let result = min_window(s, t);
        assert_eq!(result, "".to_string());
    }


    #[test]
    fn test_entire_string_needed() {
        let s = "abc".to_string();
        let t = "cba".to_string();
        let result = min_window(s, t);
        assert_eq!(result, "abc".to_string());
    }

    #[test]
    fn test_multiple_valid_windows() {
        let s = "abcdef".to_string();
        let t = "cf".to_string();
        let result = min_window(s, t);
        assert_eq!(result, "cdef".to_string());
    }

    #[test]
    fn test_case_sensitivity() {
        let s = "Abc".to_string();
        let t = "bc".to_string();
        let result = min_window(s, t);
        assert_eq!(result, "bc".to_string());
    }

    #[test]
    fn test_long_string() {
        let s = "askdjfhaskjdfhaskjdfhbc".to_string();
        let t = "abc".to_string();
        let result = min_window(s, t);
        assert_eq!(result, "askjdfhbc".to_string());
    }

    #[test]
    fn test_all_same_character() {
        let s = "aaaa".to_string();
        let t = "aa".to_string();
        let result = min_window(s, t);
        assert_eq!(result, "aa".to_string());
    }

    #[test]
    fn test_overlapping_requirements() {
        let s = "acbbaca".to_string();
        let t = "aba".to_string();
        let result = min_window(s, t);
        assert_eq!(result, "baca".to_string());
    }

    #[test]
    fn test_minimum_window_at_start() {
        let s = "abcdef".to_string();
        let t = "ab".to_string();
        let result = min_window(s, t);
        assert_eq!(result, "ab".to_string());
    }

    #[test]
    fn test_minimum_window_at_end() {
        let s = "defabc".to_string();
        let t = "abc".to_string();
        let result = min_window(s, t);
        assert_eq!(result, "abc".to_string());
    }
    
}
