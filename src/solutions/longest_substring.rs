/// https://leetcode.com/problems/longest-substring-without-repeating-characters/
use std::cmp::max;
use std::collections::HashMap;

pub struct Solution;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut longest = 0;
        let mut longest_indices = HashMap::new();
        for (i, c) in s.char_indices() {
            if let Some(p) = longest_indices.insert(c, i) {
                longest = max(longest, longest_indices.len());
                longest_indices.retain(|_, &mut v| v > p);
            }
        }
        max(longest, longest_indices.len()) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(0, Solution::length_of_longest_substring("".to_string()));
    }

    #[test]
    fn test_2() {
        assert_eq!(3, Solution::length_of_longest_substring("abcabcbb".to_string()));
    }

    #[test]
    fn test_3() {
        assert_eq!(1, Solution::length_of_longest_substring("bb".to_string()));
    }

    #[test]
    fn test_4() {
        assert_eq!(3, Solution::length_of_longest_substring("pwwkew".to_string()));
    }
}
