/// https://leetcode.com/problems/zigzag-conversion/

pub struct Solution;
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let num_rows = num_rows as usize;
        if num_rows < 2 {
            return s;
        }

        let mut result = String::with_capacity(s.len());
        let s_chars: Vec<_> = s.chars().collect();
        let cycle_len = 2 * num_rows - 2;
        let n = s_chars.len();
        for i in 0..num_rows {
            for j in (i..n).step_by(cycle_len) {
                result.push(s_chars[j]);
                if 0 < i && i < num_rows - 1 && j + cycle_len - 2 * i < n {
                    result.push(s_chars[j + cycle_len - 2 * i]);
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            "PAHNAPLSIIGYIR".to_string(),
            Solution::convert("PAYPALISHIRING".to_string(), 3)
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            "PINALSIGYAHRPI".to_string(),
            Solution::convert("PAYPALISHIRING".to_string(), 4)
        );
    }
}
