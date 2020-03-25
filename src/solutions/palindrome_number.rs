/// https://leetcode.com/problems/palindrome-number/
use std::cmp::Ordering;
pub struct Solution;
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        match x.cmp(&0) {
            Ordering::Less => false,
            Ordering::Equal => true,
            _ => {
                let mut reversed = 0;
                let mut rem = x;
                loop {
                    reversed = reversed * 10 + (rem % 10);
                    rem /= 10;
                    if rem == 0 {
                        break;
                    }
                }

                x == reversed
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(false, Solution::is_palindrome(42));
    }

    #[test]
    fn test_2() {
        assert_eq!(true, Solution::is_palindrome(242));
    }

    #[test]
    fn test_3() {
        assert_eq!(false, Solution::is_palindrome(-121));
    }
}
