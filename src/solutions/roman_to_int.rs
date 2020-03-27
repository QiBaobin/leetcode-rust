/// https://leetcode.com/problems/roman-to-integer/

pub struct Solution;
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let s = s
            .replace("IV", "IIII")
            .replace("IX", "VIIII")
            .replace("XL", "XXXX")
            .replace("XC", "LXXXX")
            .replace("CD", "CCCC")
            .replace("CM", "DCCCC");
        let mut i = 0;
        for c in s.chars() {
            i += match c {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => 0,
            }
        }
        i
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, Solution::roman_to_int("III".to_string()));
    }

    #[test]
    fn test_2() {
        assert_eq!(9, Solution::roman_to_int("IX".to_string()));
    }

    #[test]
    fn test_3() {
        assert_eq!(58, Solution::roman_to_int("LVIII".to_string()));
    }

    #[test]
    fn test_4() {
        assert_eq!(1994, Solution::roman_to_int("MCMXCIV".to_string()));
    }
}
