/// https://leetcode.com/problems/next-permutation/

struct Solution;
impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut stack = vec![];
        let mut ranges: Vec<(usize, usize)> = vec![];
        for (i, c) in s.char_indices() {
            match c {
                '(' => stack.push(i),
                ')' => {
                    if let Some(last) = stack.pop() {
                        let mut start = last;
                        while let Some(range) = ranges.pop() {
                            if range.1 + 1 >= start {
                                start = std::cmp::min(range.0, start);
                            } else {
                                ranges.push(range);
                                break;
                            }
                        }
                        ranges.push((start, i));
                    }
                }
                _ => {}
            }
        }

        ranges.iter().map(|r| r.1 - r.0 + 1).max().unwrap_or(0) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::longest_valid_parentheses(")()".to_string()), 2);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::longest_valid_parentheses(")()())".to_string()), 4);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::longest_valid_parentheses("".to_string()), 0);
    }

    #[test]
    fn test_4() {
        assert_eq!(
            Solution::longest_valid_parentheses("(())())".to_string()),
            6
        );
    }

    #[test]
    fn test_5() {
        assert_eq!(Solution::longest_valid_parentheses("(()".to_string()), 2);
    }

    #[test]
    fn test_6() {
        assert_eq!(Solution::longest_valid_parentheses("()(()".to_string()), 2);
    }
}
