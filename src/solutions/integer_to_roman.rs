/// https://leetcode.com/problems/integer-to-roman/

pub struct Solution;
impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let symbols: Vec<_> = "MDCLXVI".chars().collect();
        let mut s = String::new();
        let mut i = 0;
        let mut r = 1000;
        let mut n = num;
        while n != 0 {
            let digit = n / r;
            match digit {
                1..=3 => {
                    for _ in 0..digit {
                        s.push(symbols[i]);
                    }
                }
                4 => {
                    s.push(symbols[i]);
                    s.push(symbols[i - 1]);
                }
                5..=8 => {
                    s.push(symbols[i - 1]);
                    for _ in 5..digit {
                        s.push(symbols[i]);
                    }
                }
                9 => {
                    s.push(symbols[i]);
                    s.push(symbols[i - 2]);
                }
                _ => {}
            }
            n %= r;
            r /= 10;
            i += 2;
        }

        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!("III".to_string(), Solution::int_to_roman(3));
    }

    #[test]
    fn test_2() {
        assert_eq!("IX".to_string(), Solution::int_to_roman(9));
    }

    #[test]
    fn test_3() {
        assert_eq!("LVIII".to_string(), Solution::int_to_roman(58));
    }

    #[test]
    fn test_4() {
        assert_eq!("MCMXCIV".to_string(), Solution::int_to_roman(1994));
    }
}
