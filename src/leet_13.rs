use std::collections::HashMap;

pub fn roman_to_int(s: String) -> i32 {
    // TODO: Implement your solution here
    let chars = s.chars().collect::<Vec<char>>();
    
    let map: HashMap<char, i32> = [
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ].into_iter().collect();

    let mut ret = 0;
    let mut prev = 0;
    for char in chars.iter().rev(){
        let val = *map.get(char).unwrap();
        if val < prev{
            ret -= val
        }else{
            ret += val
        }
        prev = val;
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_single_symbols() {
        assert_eq!(roman_to_int("I".to_string()), 1);
        assert_eq!(roman_to_int("V".to_string()), 5);
        assert_eq!(roman_to_int("X".to_string()), 10);
        assert_eq!(roman_to_int("L".to_string()), 50);
        assert_eq!(roman_to_int("C".to_string()), 100);
        assert_eq!(roman_to_int("D".to_string()), 500);
        assert_eq!(roman_to_int("M".to_string()), 1000);
    }

    #[test]
    fn test_basic_addition() {
        assert_eq!(roman_to_int("II".to_string()), 2);
        assert_eq!(roman_to_int("III".to_string()), 3);
        assert_eq!(roman_to_int("VI".to_string()), 6);
        assert_eq!(roman_to_int("VII".to_string()), 7);
        assert_eq!(roman_to_int("VIII".to_string()), 8);
    }

    #[test]
    fn test_basic_subtraction() {
        assert_eq!(roman_to_int("IV".to_string()), 4);
        assert_eq!(roman_to_int("IX".to_string()), 9);
        assert_eq!(roman_to_int("XL".to_string()), 40);
        assert_eq!(roman_to_int("XC".to_string()), 90);
        assert_eq!(roman_to_int("CD".to_string()), 400);
        assert_eq!(roman_to_int("CM".to_string()), 900);
    }

    #[test]
    fn test_teens_and_twenties() {
        assert_eq!(roman_to_int("XI".to_string()), 11);
        assert_eq!(roman_to_int("XII".to_string()), 12);
        assert_eq!(roman_to_int("XIII".to_string()), 13);
        assert_eq!(roman_to_int("XIV".to_string()), 14);
        assert_eq!(roman_to_int("XV".to_string()), 15);
        assert_eq!(roman_to_int("XIX".to_string()), 19);
        assert_eq!(roman_to_int("XX".to_string()), 20);
    }

    #[test]
    fn test_common_numbers() {
        assert_eq!(roman_to_int("LVIII".to_string()), 58); // L = 50, V = 5, III = 3
        assert_eq!(roman_to_int("MCMXC".to_string()), 1990); // M = 1000, CM = 900, XC = 90
        assert_eq!(roman_to_int("MMXIV".to_string()), 2014); // MM = 2000, XIV = 14
    }

    #[test]
    fn test_complex_subtraction_cases() {
        assert_eq!(roman_to_int("MCDXLIV".to_string()), 1444); // M + CD + XL + IV = 1000 + 400 + 40 + 4
        assert_eq!(roman_to_int("CMXCIX".to_string()), 999); // CM + XC + IX = 900 + 90 + 9
        assert_eq!(roman_to_int("CDXLIV".to_string()), 444); // CD + XL + IV = 400 + 40 + 4
    }

    #[test]
    fn test_mixed_addition_subtraction() {
        assert_eq!(roman_to_int("DCXXI".to_string()), 621); // D + C + XX + I = 500 + 100 + 20 + 1
        assert_eq!(roman_to_int("MCDLIV".to_string()), 1454); // M + CD + L + IV = 1000 + 400 + 50 + 4
        assert_eq!(roman_to_int("MCMLIV".to_string()), 1954); // M + CM + L + IV = 1000 + 900 + 50 + 4
    }

    #[test]
    fn test_large_numbers() {
        assert_eq!(roman_to_int("MMM".to_string()), 3000);
        assert_eq!(roman_to_int("MMMCMXCIX".to_string()), 3999); // Largest valid Roman numeral
        assert_eq!(roman_to_int("MMDCCCLXXXVIII".to_string()), 2888); // MM + DCCC + LXXX + VIII
    }

    #[test]
    fn test_consecutive_subtractions() {
        assert_eq!(roman_to_int("XLIV".to_string()), 44); // XL + IV = 40 + 4
        assert_eq!(roman_to_int("XCIX".to_string()), 99); // XC + IX = 90 + 9
        assert_eq!(roman_to_int("CDIX".to_string()), 409); // CD + IX = 400 + 9
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(roman_to_int("MMMCMXC".to_string()), 3990); // MMM + CM + XC
        assert_eq!(roman_to_int("MMMCDXLIV".to_string()), 3444); // MMM + CD + XL + IV
        assert_eq!(roman_to_int("DCCCLXXXVIII".to_string()), 888); // DCCC + LXXX + VIII
    }

    #[test]
    fn test_all_subtraction_pairs() {
        assert_eq!(roman_to_int("MCMXCIV".to_string()), 1994); //M = 1000, CM = 900, XC = 90 and IV = 4.
        // Note: This is not a valid Roman numeral format, but tests all subtraction cases
    }

    #[test]
    fn test_repetition_patterns() {
        assert_eq!(roman_to_int("XXXIII".to_string()), 33); // XXX + III = 30 + 3
        assert_eq!(roman_to_int("LXXXVIII".to_string()), 88); // L + XXX + V + III = 50 + 30 + 5 + 3
        assert_eq!(roman_to_int("CCCXLV".to_string()), 345); // CCC + XL + V = 300 + 40 + 5
    }

    #[test]
    fn test_tricky_sequences() {
        assert_eq!(roman_to_int("MCMIV".to_string()), 1904); // M + CM + IV = 1000 + 900 + 4
        assert_eq!(roman_to_int("MDCXIV".to_string()), 1614); // M + D + C + XIV = 1000 + 500 + 100 + 14
        assert_eq!(roman_to_int("MMCDXLIX".to_string()), 2449); // MM + CD + XL + IX = 2000 + 400 + 40 + 9
    }

    #[test]
    fn test_benchmark_cases() {
        // Common test cases that might cause performance issues with naive implementations
        assert_eq!(roman_to_int("MMMCMXCIX".to_string()), 3999);
        assert_eq!(roman_to_int("MMMDCCCLXXXVIII".to_string()), 3888);
        assert_eq!(roman_to_int("MMMDCCCXC".to_string()), 3890);
    }

    #[test]
    fn test_minimum_maximum() {
        assert_eq!(roman_to_int("I".to_string()), 1); // Minimum Roman numeral
        assert_eq!(roman_to_int("MMMCMXCIX".to_string()), 3999); // Maximum Roman numeral (typically)
    }
}