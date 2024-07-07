/*
    Leetcode 13 - Roman to Integer

    Symbol       Value
    I             1
    V             5
    X             10
    L             50
    C             100
    D             500
    M             1000


    I can be placed before V (5) and X (10) to make 4 and 9. 
    X can be placed before L (50) and C (100) to make 40 and 90. 
    C can be placed before D (500) and M (1000) to make 400 and 900.

    Constraints:

    1 <= s.length <= 15
    s contains only the characters ('I', 'V', 'X', 'L', 'C', 'D', 'M').
    It is guaranteed that s is a valid roman numeral in the range [1, 3999].

*/



fn main() {
    println!("Hello, world!");
}

fn match_roman_to_int(s: &str) -> u32 {
    match s {
        "I" => 1,
        "V" => 5,
        "X" => 10,
        "L" => 50,
        "C" => 100,
        "D" => 500,
        "M" => 1000,
        _ => 0
    }
}

fn add_roman_numerals(s: &str) -> u32 {
    let mut result = 0;
    for n in 0..s.chars().count() {
        // if s == I || .. X || .. C
        let c = s.chars().nth(n).unwrap();
        let substract = false;

        if (c == 'I' || c == 'X' || c == 'C') && s.chars().nth(n+1).is_some() {
            let c_plus_one = s.chars().nth(n+1).unwrap(); {}

            match c {
                'I' => todo!(),
                'X' => todo!(),
                'C' => todo!(),
                _ => break
            }
        }
        result = result + match_roman_to_int(c.to_string().as_str());
    }
    return result;
}

fn is_relevant_char_for_substraction(previous: &char, c: &char) -> bool {
    if previous.eq_ignore_ascii_case('I') && (c == 'V' || c == 'X') {
        true
    } else if previous == 'X' && (c == 'L' || c == 'C') {
        true
    } else if previous == 'C' && (c == 'D' || c == 'M') {
        true
    } else {
        false
    }

}
 
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn match_roman_one_to_int_test() {
        let s = "I";
        assert_eq!(match_roman_to_int(s), 1);
    }

    #[test]
    fn match_roman_five_to_int_test() {
        let s = "V";
        assert_eq!(match_roman_to_int(s), 5);
    }

    #[test]
    fn match_roman_ten_to_int_test() {
        let s = "X";
        assert_eq!(match_roman_to_int(s), 10);
    }

    #[test]
    fn match_roman_fifty_to_int_test() {
        let s = "L";
        assert_eq!(match_roman_to_int(s), 50);
    }

    #[test]
    fn match_roman_hundred_to_int_test() {
        let s = "C";
        assert_eq!(match_roman_to_int(s), 100);
    }

    #[test]
    fn match_roman_fivehundred_to_int_test() {
        let s = "D";
        assert_eq!(match_roman_to_int(s), 500);
    }

    #[test]
    fn match_roman_thousand_to_int_test() {
        let s = "M";
        assert_eq!(match_roman_to_int(s), 1000);
    }

    #[test]
    fn roman_three_test() {
        let s = "III";
        assert_eq!(add_roman_numerals(s), 3);
    }

    #[test]
    fn roman_27_test() {
        let s = "XXVII";
        assert_eq!(add_roman_numerals(s), 27);
    }

    #[test]
    fn roman_four_test() {
        let s = "IV";
        assert_eq!(add_roman_numerals(s), 4);
    }
}