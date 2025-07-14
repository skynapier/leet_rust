fn my_atoi(s: String) -> i32 {
    let mut chars = s.trim().chars();

    if let Some(first_char) = chars.next() {
        let is_sign = matches!(first_char, '-' | '+');
        let is_numeric = first_char.is_numeric();

        if !is_numeric && !is_sign {
            return 0;
        }

        let mut ret = if !is_sign && is_numeric {
            first_char.to_digit(10).unwrap() as i32
        } else {
            0 as i32
        };
        

        while let Some(remaining_char) = chars.next() {
            if remaining_char.is_digit(10) {
                let digit = remaining_char.to_digit(10).unwrap() as i32;
                if ret > (i32::MAX - digit) / 10 {
                    if is_sign && first_char == '-' {
                        return -i32::MAX - 1;
                    }
                    return i32::MAX;
                }
                ret = ret * 10 + digit;
            } else {
                break;
            }
        }

        if is_sign && first_char == '-' {
            return -ret;
        } else {
            return ret;
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_positive() {
        assert_eq!(my_atoi("42".to_string()), 42);
        assert_eq!(my_atoi("123".to_string()), 123);
    }

    #[test]
    fn test_basic_negative() {
        assert_eq!(my_atoi("-42".to_string()), -42);
        assert_eq!(my_atoi("-123".to_string()), -123);
    }

    #[test]
    fn test_with_leading_whitespace() {
        assert_eq!(my_atoi("   42".to_string()), 42);
        assert_eq!(my_atoi("  -42".to_string()), -42);
        assert_eq!(my_atoi("\t\n\r 123".to_string()), 123);
    }

    #[test]
    fn test_with_trailing_characters() {
        assert_eq!(my_atoi("42abc".to_string()), 42);
        assert_eq!(my_atoi("-42def".to_string()), -42);
        assert_eq!(my_atoi("123 456".to_string()), 123);
    }

    #[test]
    fn test_invalid_input() {
        assert_eq!(my_atoi("abc".to_string()), 0);
        assert_eq!(my_atoi("".to_string()), 0);
        assert_eq!(my_atoi("   ".to_string()), 0);
        assert_eq!(my_atoi(".1".to_string()), 0);
        assert_eq!(my_atoi("+-12".to_string()), 0);
    }

    #[test]
    fn test_overflow() {
        // Test positive overflow
        assert_eq!(my_atoi("2147483648".to_string()), i32::MAX); // 2^31
        assert_eq!(my_atoi("99999999999999999999".to_string()), i32::MAX);

        // Test negative overflow
        assert_eq!(my_atoi("-2147483649".to_string()), i32::MIN); // -2^31 - 1
        assert_eq!(my_atoi("-99999999999999999999".to_string()), i32::MIN);
    }

    #[test]
    fn test_boundary_values() {
        assert_eq!(my_atoi("2147483647".to_string()), 2147483647); // i32::MAX
        assert_eq!(my_atoi("-2147483648".to_string()), -2147483648); // i32::MIN
    }

    #[test]
    fn test_zero() {
        assert_eq!(my_atoi("0".to_string()), 0);
        assert_eq!(my_atoi("-0".to_string()), 0);
        assert_eq!(my_atoi("+0".to_string()), 0);
        assert_eq!(my_atoi("000".to_string()), 0);
    }

    #[test]
    fn test_positive_sign() {
        assert_eq!(my_atoi("+42".to_string()), 42);
        assert_eq!(my_atoi("+123".to_string()), 123);
    }

    #[test]
    fn test_mixed_whitespace_and_signs() {
        assert_eq!(my_atoi("  +42".to_string()), 42);
        assert_eq!(my_atoi("  -42".to_string()), -42);
        assert_eq!(my_atoi("42 ".to_string()), 42); // trailing whitespace after number
    }
}