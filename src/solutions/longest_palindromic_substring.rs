/// https://leetcode.com/problems/longest-palindromic-substring/
use std::cmp::min;

pub struct Solution;
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let s: Vec<char> = s.chars().collect();
        let n = s.len() * 2 + 1;
        let mut pls = Vec::with_capacity(n); // palindromic radius on each point, includes chars and the points between them
        let (mut max_pl, mut max_pl_index) = (0, 0);

        let mut center = 0;
        pls.insert(0, 0); //the point before first char
        for i in 1..n {
            let mut j; // the right index to calculate
            let right = center + pls[center];
            if right > i {
                let mirror = 2 * center - i;
                let left_mirror = mirror - pls[mirror];
                let left = center - pls[center];
                if left_mirror > left {
                    // palindrome is wrapped
                    pls.insert(i, pls[mirror]);
                    continue;
                }

                // calculate the part outside
                j = right;
            } else {
                // center reaching to the right farthest
                j = i;
            }

            let end = min(n, 2 * i + 1); // most step to reach array bound
            while j < end && ((j & 1) == 0 || s[j / 2] == s[(2 * i - j) / 2]) {
                j += 1;
            }
            pls.insert(i, j - i - 1);
            if max_pl < pls[i] {
                max_pl = pls[i];
                max_pl_index = i;
            }
            if max_pl + i > n {
                // no chance to get a longer one
                break;
            }
            center = i;
        }
        s[((max_pl_index - max_pl) / 2)..((max_pl_index + max_pl) / 2)]
            .iter()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!("".to_string(), Solution::longest_palindrome("".to_string()));
    }

    #[test]
    fn test_2() {
        assert_eq!(
            "b".to_string(),
            Solution::longest_palindrome("b".to_string())
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            "a".to_string(),
            Solution::longest_palindrome("ab".to_string())
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            "aba".to_string(),
            Solution::longest_palindrome("aaba".to_string())
        );
    }

    #[test]
    fn test_5() {
        assert_eq!(
            "anana".to_string(),
            Solution::longest_palindrome("banana".to_string())
        );
    }

    #[test]
    fn test_6() {
        assert_eq!(
            "bb".to_string(),
            Solution::longest_palindrome("cbba".to_string())
        );
    }
}
