/// https://leetcode.com/problems/valid-parentheses/

struct Solution;
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = vec![];
        for c in s.chars() {
            match c {
                '(' => stack.push(')'),
                '{' => stack.push('}'),
                '[' => stack.push(']'),
                ')' | '}' | ']' => {
                    if stack.pop() != Some(c) {
                        return false;
                    }
                }
                _ => return false,
            }
        }
        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(true, Solution::is_valid("(){[]}".to_string()));
    }

    #[test]
    fn test_2() {
        assert_eq!(false, Solution::is_valid("([)]".to_string()));
    }
}
