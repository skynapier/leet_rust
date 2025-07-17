struct Leet7;

impl Leet7 {
    pub fn reverse(x: i32) -> i32 {
        let is_neg = x < 0;
        let mut org = x.abs();
        let mut result = 0;

        while org > 0 {
            let digit = org % 10;
            if result > (i32::MAX - digit) / 10 {
                return 0;
            }
            result = result * 10 + digit;
            org = org / 10;
        }

        if is_neg {
            -result
        } else {
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_positive() {
        assert_eq!(Leet7::reverse(123), 321);
        assert_eq!(Leet7::reverse(120), 21);
        assert_eq!(Leet7::reverse(1), 1);
    }

    #[test]
    fn test_basic_negative() {
        assert_eq!(Leet7::reverse(-123), -321);
        assert_eq!(Leet7::reverse(-120), -21);
        assert_eq!(Leet7::reverse(-1), -1);
    }

    #[test]
    fn test_zero() {
        assert_eq!(Leet7::reverse(0), 0);
    }

    #[test]
    fn test_single_digit() {
        assert_eq!(Leet7::reverse(7), 7);
        assert_eq!(Leet7::reverse(-7), -7);
    }

    #[test]
    fn test_trailing_zeros() {
        assert_eq!(Leet7::reverse(1000), 1);
        assert_eq!(Leet7::reverse(10200), 201);
        assert_eq!(Leet7::reverse(-1000), -1);
    }

    #[test]
    fn test_overflow_positive() {
        // Test cases that would overflow when reversed
        assert_eq!(Leet7::reverse(1534236469), 0); // reversed would be 9646324351 > i32::MAX
    }

    #[test]
    fn test_overflow_negative() {
        // Test cases that would underflow when reversed
        assert_eq!(Leet7::reverse(-1563847412), 0); 
    }

    #[test]
    fn test_boundary_values() {
        // Test values close to i32 boundaries
        assert_eq!(Leet7::reverse(1463847412), 2147483641); // Valid reverse
        assert_eq!(Leet7::reverse(-1463847412), -2147483641); // Valid reverse
    }

    #[test]
    fn test_palindromes() {
        assert_eq!(Leet7::reverse(121), 121);
        assert_eq!(Leet7::reverse(-121), -121);
        assert_eq!(Leet7::reverse(1221), 1221);
    }

    #[test]
    fn test_large_numbers() {
        assert_eq!(Leet7::reverse(1234567), 7654321);
        assert_eq!(Leet7::reverse(-1234567), -7654321);
        assert_eq!(Leet7::reverse(987654321), 123456789);
    }

    #[test]
    fn test_edge_cases() {
        // Numbers with many trailing zeros
        assert_eq!(Leet7::reverse(54000), 45);
        assert_eq!(Leet7::reverse(-54000), -45);

        // Numbers that start with zero when reversed
        assert_eq!(Leet7::reverse(1200), 21);
        assert_eq!(Leet7::reverse(-1200), -21);
    }
}