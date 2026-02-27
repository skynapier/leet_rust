use std::collections::HashMap;

pub fn int_to_roman(num: i32) -> String {
    // TODO: Implement your solution here

    let map: HashMap<i32, &str> = [
        (1, "I"),
        (5, "V"),
        (10, "X"),
        (50, "L"),
        (100, "C"),
        (500, "D"),
        (1000, "L")
    ].into_iter().collect();
    
    let mut ret = String::new();
    let mut num = num;
    while num > 0{
        let digit = num %10;
        ret = format!("{}{}", digit, ret );
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_single_digits() {
        assert_eq!(int_to_roman(1), "I");
        assert_eq!(int_to_roman(2), "II");
        assert_eq!(int_to_roman(3), "III");
        assert_eq!(int_to_roman(4), "IV");
        assert_eq!(int_to_roman(5), "V");
        assert_eq!(int_to_roman(6), "VI");
        assert_eq!(int_to_roman(7), "VII");
        assert_eq!(int_to_roman(8), "VIII");
        assert_eq!(int_to_roman(9), "IX");
    }

    #[test]
    fn test_basic_tens() {
        assert_eq!(int_to_roman(10), "X");
        assert_eq!(int_to_roman(20), "XX");
        assert_eq!(int_to_roman(30), "XXX");
        assert_eq!(int_to_roman(40), "XL");
        assert_eq!(int_to_roman(50), "L");
        assert_eq!(int_to_roman(60), "LX");
        assert_eq!(int_to_roman(70), "LXX");
        assert_eq!(int_to_roman(80), "LXXX");
        assert_eq!(int_to_roman(90), "XC");
    }

    #[test]
    fn test_basic_hundreds() {
        assert_eq!(int_to_roman(100), "C");
        assert_eq!(int_to_roman(200), "CC");
        assert_eq!(int_to_roman(300), "CCC");
        assert_eq!(int_to_roman(400), "CD");
        assert_eq!(int_to_roman(500), "D");
        assert_eq!(int_to_roman(600), "DC");
        assert_eq!(int_to_roman(700), "DCC");
        assert_eq!(int_to_roman(800), "DCCC");
        assert_eq!(int_to_roman(900), "CM");
    }

    #[test]
    fn test_basic_thousands() {
        assert_eq!(int_to_roman(1000), "M");
        assert_eq!(int_to_roman(2000), "MM");
        assert_eq!(int_to_roman(3000), "MMM");
    }

    #[test]
    fn test_teens_and_twenties() {
        assert_eq!(int_to_roman(11), "XI");
        assert_eq!(int_to_roman(12), "XII");
        assert_eq!(int_to_roman(13), "XIII");
        assert_eq!(int_to_roman(14), "XIV");
        assert_eq!(int_to_roman(15), "XV");
        assert_eq!(int_to_roman(16), "XVI");
        assert_eq!(int_to_roman(17), "XVII");
        assert_eq!(int_to_roman(18), "XVIII");
        assert_eq!(int_to_roman(19), "XIX");
        assert_eq!(int_to_roman(21), "XXI");
    }

    #[test]
    fn test_common_examples() {
        assert_eq!(int_to_roman(58), "LVIII"); // L + V + III
        assert_eq!(int_to_roman(1994), "MCMXCIV"); // M + CM + XC + IV
        assert_eq!(int_to_roman(3749), "MMMDCCXLIX"); // MMM + DCC + XL + IX
    }

    #[test]
    fn test_subtraction_cases() {
        assert_eq!(int_to_roman(44), "XLIV"); // XL + IV
        assert_eq!(int_to_roman(49), "XLIX"); // XL + IX
        assert_eq!(int_to_roman(94), "XCIV"); // XC + IV
        assert_eq!(int_to_roman(99), "XCIX"); // XC + IX
        assert_eq!(int_to_roman(444), "CDXLIV"); // CD + XL + IV
        assert_eq!(int_to_roman(449), "CDXLIX"); // CD + XL + IX
        assert_eq!(int_to_roman(494), "CDXCIV"); // CD + XC + IV
        assert_eq!(int_to_roman(499), "CDXCIX"); // CD + XC + IX
    }

    #[test]
    fn test_complex_combinations() {
        assert_eq!(int_to_roman(1444), "MCDXLIV"); // M + CD + XL + IV
        assert_eq!(int_to_roman(1954), "MCMLIV"); // M + CM + L + IV
        assert_eq!(int_to_roman(1990), "MCMXC"); // M + CM + XC
        assert_eq!(int_to_roman(2014), "MMXIV"); // MM + XIV
        assert_eq!(int_to_roman(2421), "MMCDXXI"); // MM + CD + XX + I
    }

    #[test]
    fn test_all_nines() {
        assert_eq!(int_to_roman(9), "IX");
        assert_eq!(int_to_roman(99), "XCIX");
        assert_eq!(int_to_roman(999), "CMXCIX");
        assert_eq!(int_to_roman(3999), "MMMCMXCIX"); // Maximum value
    }

    #[test]
    fn test_all_fours() {
        assert_eq!(int_to_roman(4), "IV");
        assert_eq!(int_to_roman(44), "XLIV");
        assert_eq!(int_to_roman(444), "CDXLIV");
        assert_eq!(int_to_roman(3444), "MMMCDXLIV");
    }

    #[test]
    fn test_boundary_values() {
        assert_eq!(int_to_roman(1), "I"); // Minimum value
        assert_eq!(int_to_roman(3999), "MMMCMXCIX"); // Maximum value
    }

    #[test]
    fn test_mid_range_numbers() {
        assert_eq!(int_to_roman(27), "XXVII");
        assert_eq!(int_to_roman(621), "DCXXI");
        assert_eq!(int_to_roman(1776), "MDCCLXXVI");
        assert_eq!(int_to_roman(2023), "MMXXIII");
    }

    #[test]
    fn test_repeated_symbols() {
        assert_eq!(int_to_roman(33), "XXXIII"); // XXX + III
        assert_eq!(int_to_roman(88), "LXXXVIII"); // L + XXX + V + III
        assert_eq!(int_to_roman(333), "CCCXXXIII"); // CCC + XXX + III
        assert_eq!(int_to_roman(888), "DCCCLXXXVIII"); // DCCC + LXXX + VIII
    }

    #[test]
    fn test_no_subtraction_needed() {
        assert_eq!(int_to_roman(123), "CXXIII"); // C + XX + III
        assert_eq!(int_to_roman(567), "DLXVII"); // D + L + X + V + II
        assert_eq!(int_to_roman(1237), "MCCXXXVII"); // M + CC + XXX + VII
        assert_eq!(int_to_roman(2678), "MMDCLXXVIII"); // MM + DC + LXX + VIII
    }

    #[test]
    fn test_maximum_repetitions() {
        assert_eq!(int_to_roman(3888), "MMMDCCCLXXXVIII"); // MMM + DCCC + LXXX + VIII
        assert_eq!(int_to_roman(3333), "MMMCCCXXXIII"); // MMM + CCC + XXX + III
        assert_eq!(int_to_roman(1888), "MDCCCLXXXVIII"); // M + DCCC + LXXX + VIII
    }

    #[test]
    fn test_special_years() {
        assert_eq!(int_to_roman(1066), "MLXVI"); // Battle of Hastings
        assert_eq!(int_to_roman(1492), "MCDXCII"); // Columbus
        assert_eq!(int_to_roman(1776), "MDCCLXXVI"); // American Independence
        assert_eq!(int_to_roman(1969), "MCMLXIX"); // Moon landing
        assert_eq!(int_to_roman(2000), "MM"); // Millennium
    }

    #[test]
    fn test_edge_subtraction_patterns() {
        assert_eq!(int_to_roman(904), "CMIV"); // CM + IV
        assert_eq!(int_to_roman(909), "CMIX"); // CM + IX
        assert_eq!(int_to_roman(940), "CMXL"); // CM + XL
        assert_eq!(int_to_roman(990), "CMXC"); // CM + XC
        assert_eq!(int_to_roman(994), "CMXCIV"); // CM + XC + IV
    }

    #[test]
    fn test_benchmark_cases() {
        // Performance test cases
        assert_eq!(int_to_roman(3749), "MMMDCCXLIX");
        assert_eq!(int_to_roman(3888), "MMMDCCCLXXXVIII");
        assert_eq!(int_to_roman(3994), "MMMCMXCIV");
        assert_eq!(int_to_roman(3999), "MMMCMXCIX");
    }

    #[test]
    fn test_random_comprehensive() {
        assert_eq!(int_to_roman(246), "CCXLVI");
        assert_eq!(int_to_roman(789), "DCCLXXXIX");
        assert_eq!(int_to_roman(1357), "MCCCLVII");
        assert_eq!(int_to_roman(2468), "MMCDLXVIII");
        assert_eq!(int_to_roman(3579), "MMMDLXXIX");
    }
}