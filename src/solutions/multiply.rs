// https://leetcode.com/problems/multiply-strings/

struct Solution;
impl Solution {
    pub fn multiply(mut num1: String, mut num2: String) -> String {
        let (base, times) = if num1.len() >= num2.len() {
            (num1, num2)
        } else {
            (num2, num1)
        };
        let mut stack: Vec<u32> = vec![0; base.len() + times.len()];
        let base: Vec<u32> = base
            .chars()
            .rev()
            .map(|c| c.to_digit(10).unwrap())
            .collect();
        for (i, c) in times.chars().rev().enumerate() {
            if let Some(n) = c.to_digit(10) {
                if n != 0 {
                    let mut carry = 0;
                    for (j, m) in base.iter().enumerate() {
                        let r = m * n + stack[i + j] + carry;
                        stack[i + j] = r % 10;
                        carry = r / 10;
                    }
                    stack[i + base.len()] += carry;
                }
            }
        }

        let ret: String = stack
            .iter()
            .rev()
            .skip_while(|&n| *n == 0)
            .map(|&n| std::char::from_digit(n, 10).unwrap())
            .collect();
        if ret.is_empty() {
            "0".to_string()
        } else {
            ret
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::multiply("123".to_string(), "456".to_string()),
            "56088".to_string()
        );
    }
}
